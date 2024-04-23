use std::{
    env,
    process,
    time::Instant
};

pub mod enc;
use enc::encrypt;

pub mod support;
use support::*;

pub mod dec;
use dec::decrypt;


fn main() {
    let now = Instant::now();

    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Error, you must provide the right number of arguments");
        process::exit(1);
    } else {
        match args[1].as_str() {
            "-e" => {
                // let password = ask_password();
                let password = hash_password("password"); //provvisorio
                encrypt(&password, args[2].as_str());
            },
            "-d" => {
                // let password = ask_password();
                let password = hash_password("password"); //provvisorio
                decrypt(&password, args[2].as_str());
            },
            _ => {
                eprintln!("Error, invalid option");
                process::exit(1);
            },
        }
    }
    println!("fine: {:?}", now.elapsed());
}

/* 
usage:
cargo run --release -- OPTION [file] 
    OPTION
    -e => encrypt
    -d => decrypt
*/