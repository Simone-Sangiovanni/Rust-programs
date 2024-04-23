/* Generate a random 20 characters password */

use rand::{Rng, thread_rng};
use std::process::ExitCode;
use std::env;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Error: you must specify an argument.");
        ExitCode::FAILURE
    } else {
        let length = match args[1].parse::<u32>() {
            Ok(lenght) => lenght,
            Err(_e) => 0,
        };
        if length == 0 {
            println!("The argumet must be a number in range drom 8 to 100 included.");
            ExitCode::FAILURE
        } else if length < 8 || length > 100 {
            println!("The length must be in range from 8 to 100 included.");
            ExitCode::FAILURE
        } else {
            generate_password(20);
            ExitCode::SUCCESS
        }
    }
}

fn generate_password(psw_length: u32) {
    let mut seed = thread_rng();

    let numbers = "0123456789";
    let lowercase_letters = "abcdefghijklmnoqprstuvwyzx";
    let capital_letters = "ABCDEFGHIJKLMNOQPRSTUYWVZX";
    let special_characters = "!@#$^&*?";
    let mut password = String::from("");
    for _ in 0..psw_length {
        let randomizer = seed.gen_range(1..=4);
        if randomizer == 1 {
            let letter = numbers.chars().nth(seed.gen_range(0..=9)).unwrap();
            password.push(letter);
        } else if randomizer == 2 {
            let letter = lowercase_letters.chars().nth(seed.gen_range(0..=25)).unwrap();
            password.push(letter);
        } else if randomizer == 3 {
            let letter = capital_letters.chars().nth(seed.gen_range(0..=25)).unwrap();
            password.push(letter);
        } else {
            let letter = special_characters.chars().nth(seed.gen_range(0..=7)).unwrap();
            password.push(letter);
        }
    }
    println!("{}", password);
}