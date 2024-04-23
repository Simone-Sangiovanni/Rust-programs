use std::{
    env,
    io::{Read, BufReader},
    fs::File,
    process, fmt::Error
};
use std::time::Instant;




fn main() -> Result<(), Error>{
    let now = Instant::now();

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Error, you must provide the right number of arguments");
        process::exit(1);
    } else {
        let file = match File::open(args[1].as_str()) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Error opening the file {}: {}", args[1].as_str(), e);
                process::exit(1);
            },
        };
        let file_len = file.metadata().unwrap().len(); //determine file length
        let mut reader = BufReader::new(file);
        let mut buffer= [0u8; 1024*512]; //buffer da 1/2 megabyte
        let mut pos = 0; //identify the number of bytes read

        while let Ok(n) = reader.read(&mut buffer) {
            if n == 0 {
                break;
            }
            if file_len - pos < 1024*512 {
                let mut x = (file_len - pos) as usize;
                while x < 1024*512 {
                    buffer[x] = 0;
                    x += 1;
                }
            }
            pos = pos + 1024*512;
        }
    }
    println!("fine: {:2?}", now.elapsed());
    Ok(())
}