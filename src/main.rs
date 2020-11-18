use std::io::{Read,Write};
use std::fs::OpenOptions;

#[derive(Debug)]
struct KSignerArgs {
    load: String, // fname to load a keypair from
    save: String, // fname to write keypair to
    public: bool, // Display public key
    sign: bool, // Sign the files
    files: Vec<String>,
}
impl KSignerArgs {
    fn new() -> Self {
        use clap::{App, Arg};
        let matches = App::new("ksigner")
            .version("0.1")
            .about("Signs files by tacking a signature onto the end")
            .long_about(
                "Signs files by tacking a signature onto the end of the file. Using ed25519 signatures (RFC 8032).
 
Can load a previously generated signature
or store a signature to a file for reuse later. ",
            )
            .arg(
                Arg::with_name("load")
                    .short("l")
                    .long("load")
                    .takes_value(true)
                    .help("Load keypair from file"),
            )
            .arg(
                Arg::with_name("save")
                    .short("s")
                    .long("save")
                    .takes_value(true)
                    .help("Save keypair to file"),
            )
            .arg(Arg::with_name("public")
                 .short("p")
                 .long("public")
                 .help("Display public key"))
            .arg(
                Arg::with_name("Sign")
                    .short("S")
                    .long("Sign")
                    .help("Sign the files"),
            )
            .arg(Arg::with_name("files")
                 .multiple(true)
                 .help("Files to operate on"))
            .get_matches();
        
        KSignerArgs {
            load: matches.value_of("load").unwrap_or("").to_string(),
            save: matches.value_of("save").unwrap_or("").to_string(),
            public: matches.is_present("public"),
            sign: matches.is_present("sign"),
            files:
            match matches.values_of("files") {
                Some(iter) => iter.map(|s|s.to_string()).collect(),
                _ => vec![],
            }
        }
    }
    
}
use ed25519_dalek::Keypair;
use std::fs::File;
fn get_keypair(s: &str) -> Keypair {
    if s=="" {
        use rand::rngs::OsRng;
        let mut csprng=OsRng{};
        Keypair::generate(&mut csprng)
    } else {
        let r=File::open(s).expect("Couldn't open key file");
        serde_json::from_reader(r).expect("Read or parse error")
    }
}


fn process_files(sign: bool, kp: &Keypair, fns: &Vec<String>) {
    use ed25519_dalek::{SIGNATURE_LENGTH, Signature, Signer};
    for fname in fns {
        if sign {
            println!("Signing  {}",fname);
            let mut r=File::open(fname).expect(&format!("Couldn't open file {}",fname));
            let mut buf=Vec::new();
            r.read_to_end(&mut buf).expect(&format!("Read error on {}",fname));
            drop(r);
            let sig=kp.sign(&buf);
            let mut w=OpenOptions::new().append(true).open(fname)
                .expect(&format!("Couln't append {}",fname));
            w.write_all(&sig.to_bytes()).expect("Write failure");
        } else {
            println!("Checking: {}",fname);
            let mut r=File::open(fname).expect("Couldn't open file");
            let len=r.metadata().unwrap().len() as usize-SIGNATURE_LENGTH;
            let mut buf=vec![0u8;len];
            r.read_exact(&mut buf).expect("Read error");
            let mut sigbuf= [0u8; SIGNATURE_LENGTH];
            r.read_exact(&mut sigbuf).expect("Read sig error");
            let sig=Signature::from(sigbuf);
            let res=kp.verify(&buf,&sig);
            if res.is_err() {
                println!("Signature failed on {}: {:?}",fname,res);
            } else{
                println!("Signature valid for {}",fname);
            }
        }
    }
}
fn save_keypair(sf: &str, kp: &Keypair){
    if sf=="" {
        return;
    }
    let w=File::create(sf).expect("Couldn't create key file");
    serde_json::to_writer(w,kp).expect("Couldn't serialize keypair");
}
fn main() {
    let args = KSignerArgs::new();
    println!("args: {:?}", args);
    let kp= get_keypair(&args.load);
    if args.public {
        println!("Public key: {}",serde_json::to_string(&kp.public).unwrap());
    }
    process_files(args.sign,&kp,&args.files);
    save_keypair(&args.save, &kp);
}
