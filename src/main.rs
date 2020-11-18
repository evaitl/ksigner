#[derive(Debug)]
struct KSignerArgs {
    load: String, // fname to load a keypair from
    save: String, // fname to write keypair to
    sign: bool, // Sign the files
    files: Vec<String>,
}
impl KSignerArgs {
    fn new() -> Self {
        use clap::{App, Arg};
        let app = App::new("ksigner")
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
            .arg(
                Arg::with_name("Sign")
                    .short("S")
                    .long("Sign")
                    .takes_value(true)
                    .help("Sign the files"),
            )
            .arg(Arg::with_name("files")
                 .multiple(true)
                 .help("Files to operate on"))
            .get_matches();
        KSignerArgs {
            load: String::new(),
            save: String::new(),
            sign: false,
            files: Vec::new(),
        }
    }
}
use ed25519_dalek::Keypair;
use std::fs::File;
use std::io::prelude::*;
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
    println!("Hello, world!");
    println!("args: {:?}", args);
    let keypair= get_keypair(&args.load);
    process_files(args.sign,&keypair,&args.files);
    save_keypair(&args.save, &keypair);
}
