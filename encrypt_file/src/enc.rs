use std::{
    io::{Read, BufReader},
    fs::{File, OpenOptions},
    process
};

use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    XChaCha20Poly1305
};

use crate::support::*;

pub fn encrypt(password: &[u8], path: &str) {
    // open input file
    let input_file = match File::open(path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening the file {}: {}", path, e);
            process::exit(1);
        },
    };

    // generate output path from input path
    let mut output_path = String::from(path);
    output_path.push_str(".enc");

    // open output_file in append mode
    let mut output_file = match OpenOptions::new().create(true).append(true).open(&output_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error opening the file {} in append mode: {}", &output_path, e);
            process::exit(1);
        },
    };

    // generate random nonce
    let nonce = XChaCha20Poly1305::generate_nonce(&mut OsRng); // 192-bits -> extended nonce

    // generate cipher from password
    let cipher = match XChaCha20Poly1305::new_from_slice(password) {
        Ok(c) => c,
        Err(_e) => {
            eprintln!("Error generating cipher");
            process::exit(1);
        },
    };

    append_data(&mut output_file, &nonce);

    let file_len = input_file.metadata().unwrap().len(); //determine file length
    let mut reader = BufReader::new(input_file);
    let mut buffer = [0u8; 1000*500]; //buffer da 1/2 megabyte
    let mut offset = 0; // number of bytes already read
    let mut ciphertext = Vec::new();

    // read input_file into buffer
    while let Ok(n) = reader.read(&mut buffer) {
        if n == 0 {
            break;
        }
        /* if the number of byte that must be read is smaller than the size of the buffer create a 
        new buffer that fit perfectly the data that remain to be read and terminate the encryption */
        if file_len - offset < 1000*500 {
            let x = (file_len - offset) as usize;
            let (small_buffer, _drop) = buffer.split_at(x);

            ciphertext.clear();
            ciphertext = match cipher.encrypt(&nonce.into(), small_buffer.as_ref()) {
                Ok(text) => text,
                Err(_e) => {
                    eprintln!("Error encrypting small buffer");
                    process::exit(1);
                },
            };
            append_data(&mut output_file, &ciphertext);
        } else {
            ciphertext.clear();
            ciphertext = match cipher.encrypt(&nonce.into(), buffer.as_ref()) {
                Ok(text) => text,
                Err(_e) => {
                    eprintln!("Error encrypting buffer");
                    process::exit(1);
                },
            };
            append_data(&mut output_file, &ciphertext);
        }
        offset = offset + 1000*500;
    }
}
