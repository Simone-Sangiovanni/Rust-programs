use std::io;
use std::process;

fn main() {
    loop{
        let choise = header();
        
        match choise {
            1 => converter_dec_to_bin_oct_hex(2),
            2 => converter_dec_to_bin_oct_hex(8),
            3 => converter_dec_to_bin_oct_hex(16),
            4 => {
                println!("Type the binary number.");
                convert_bin_oct_hex_to_dec(2);
            },
            5 => {
                println!("Type the octal number.");
                convert_bin_oct_hex_to_dec(8);
            },
            6 => {
                println!("Type the hexadecimal number.");
                convert_bin_oct_hex_to_dec(16);
            },
            7 => process::exit(1),
            _ => {continue;}
        };
        wait();
    }
}

fn header() -> u8 {
    std::process::Command::new("clear").status().unwrap();
    
    println!("--------------------------");
    println!("     NUMBER CONVERTER");
    println!("--------------------------");
    println!("  1) Decimal to Binary");
    println!("  2) Decimal to Octal");
    println!("  3) Decimal to Hexadecimal");
    println!("  4) Binary to Decimal");
    println!("  5) Octal to Decimal");
    println!("  6) Hexadecimal to Decimal");
    println!("  7) Exit");

    let choise: u8 = get_input()
        .trim()
        .parse()
        .expect("You must type a number");

    return choise;
}

fn wait() {
    loop {
        println!("Press ENTER to continue.");
        let mut proceed = String::new();
        io::stdin()
            .read_line(&mut proceed)
            .expect("Failed to read line");
        let proceed: Vec<char> = proceed.chars().collect();
        if proceed[0] != '\n' {
            continue;
        } else { break; }
    }
}

fn get_input() -> String {
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    return user_input;
}

fn get_number() -> u32 {
    println!("Write the number you want to convert (must be positive).");
    let number: u32 = get_input()
        .trim()
        .parse()
        .expect("You must type a positive number");

    return number;
}

fn converter_dec_to_bin_oct_hex(divider: u32) {
    let mut number = get_number();
    let mut vector: Vec<u32> = Vec::new(); 
    let mut rest: u32;
    loop {
        rest = number % divider;
        number = number / divider;
        vector.push(rest);
        if number == 0 {break;}
    }
    for n in vector.iter().rev() {
        if n == &10 {
            print!("A");
        } else if n == &11 {
            print!("B");
        } else if n == &12 {
            print!("C");
        } else if n == &13 {
            print!("D");
        } else if n == &14 {
            print!("E");
        } else if n == &15 {
            print!("F");
        } else {
            print!("{n}");
        }
    }
    println!();
}

fn convert_bin_oct_hex_to_dec(base: i32) {
    let input = get_input();
    let number = input.trim();
    let mut result: i32 = 0;
    let mut i = 0;
    for n in number.chars().rev() {
        match n {
            'A' => result = result + (base.pow(i) * 10),
            'B' => result = result + (base.pow(i) * 11),
            'C' => result = result + (base.pow(i) * 12),
            'D' => result = result + (base.pow(i) * 13),
            'E' => result = result + (base.pow(i) * 14),
            'F' => result = result + (base.pow(i) * 15),
            _ => result = result + (base.pow(i) * n.to_string().parse::<i32>().unwrap()),
        };
        i = i+1;
    }
    println!("{result}");
}