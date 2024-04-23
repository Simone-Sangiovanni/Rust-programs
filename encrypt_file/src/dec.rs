use std::{
    io::{Read, BufReader, SeekFrom, Seek},
    fs::{File, OpenOptions},
    process
};

use chacha20poly1305::{
    aead::{Aead, KeyInit},
    XChaCha20Poly1305
};

use crate::support::*;



pub fn decrypt(password: &[u8], path: &str) {
    // open input file
    let input_file = match File::open(path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening the file {}: {}", path, e);
            process::exit(1);
        },
    };

    // output path
    let path_len = path.len();
    let (output_path, _drop) = path.split_at(path_len - 4);

    // open output_file in append mode
    let mut output_file = match OpenOptions::new().create(true).append(true).open(&output_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error opening the file {} in append mode: {}", &output_path, e);
            process::exit(1);
        },
    };

    /* generate the cipher from the password. if the password is different from the one used during
    encryption will not be able to decrypt the file */
    let cipher = match XChaCha20Poly1305::new_from_slice(&password) {
        Ok(c) => c,
        Err(_e) => {
            eprintln!("Error generating cipher");
            process::exit(1);
        },
    };

    let file_len = input_file.metadata().unwrap().len(); //determine file length
    let mut nonce: [u8; 24] = [0; 24];
    let mut reader = BufReader::new(input_file);
    let mut buffer = [0u8; (1000*500 + 16)]; //buffer da 1/2 megabyte + i 16 byte aggiunti (ad ogni chunk) in fase di cifratura
    let mut offset = 0;
    let mut plaintext = Vec::new();

    /* read nonce -> 24 bytes */
    match reader.read(&mut nonce) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Error extracting nonce during decryption: {}", e);
            process::exit(1);
        },
    };
    offset = offset + 24;

    // change cursor position
    let _position = match reader.seek(SeekFrom::Start(offset)) {
        Ok(ok) => ok,
        Err(e) => {
            eprintln!("Error positioning offset: {}", e);
            process::exit(1);
        },
    };

    while let Ok(n) = reader.read(&mut buffer) {
        if n == 0 {
            break;
        } else {
            if (file_len - offset) < (1000*500 + 16) {
                /* read small buffer */
                let x = (file_len - offset) as usize;
                let (small_buffer, _drop) = buffer.split_at(x);
    
                plaintext.clear();
                plaintext = match cipher.decrypt(&nonce.into(), small_buffer.as_ref()) {
                    Ok(text) => text,
                    Err(e) => {
                        eprintln!("Error decrypting small buffer: {}", e);
                        process::exit(1);
                    },
                };
                append_data(&mut output_file, &plaintext);
            } else {
                /* read buffer */
                plaintext.clear();
                plaintext = match cipher.decrypt(&nonce.into(), buffer.as_ref()) {
                    Ok(text) => text,
                    Err(e) => {
                        eprintln!("Error decrypting buffer: {}", e);
                        process::exit(1);
                    },
                };
                append_data(&mut output_file, &plaintext);
                offset = offset + (1000*500+16);
            }
        }
    }
}