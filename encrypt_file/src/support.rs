use sha3::{Digest, Sha3_256};
use std::{
    fs::File, 
    process, 
    io::Write
};

// ask the password and return the hash
pub fn ask_password() -> [u8; 32] {
    /* read password from stdin without showing what is typed */
    println!("Password: ");
    let password = rpassword::read_password().unwrap();

    let hash = hash_password(&password);

    hash
}

pub fn hash_password(psw: &str) -> [u8; 32] {
    // create a SHA3-512 object
    let mut hasher = Sha3_256::new();
    // write input message
    hasher.update(psw.as_bytes());
    // read hash digest
    let result = hasher.finalize();
    let hash: [u8; 32] = result.into();

    hash
}

pub fn append_data(file: &mut File, data: &[u8]) {
    match file.write_all(data){
        Ok(file) => file,
        Err(e) => {
            eprintln!("Unable to append data to file: {}", e);
            process::exit(1);
        },
    };
}