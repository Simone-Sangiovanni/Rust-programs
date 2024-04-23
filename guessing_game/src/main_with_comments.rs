use std::io; //include the I/O library (or io module) from the standard library (or std)
use rand::Rng;
use std::cmp::Ordering; /*The Orgering type is an enum and has the variants of Less, Greater and Equal*/

fn main() { //entry point of the program
    println!("GUESS THE NUMBER"); //macro that print to screen
    
    let secret_number = rand::thread_rng() //give us the random number generator
                            .gen_range(1..=100); //generate a random number in the range specified  
    loop {
        println!("\nPlease input your guess. ");

        let mut guess = String::new(); /*create a mutable variable of type String. If we omit "mut" the 
                                         variable will be immutable. The part on the right of the equal sign means
                                         that we are creating a new string (using the new() function associated of 
                                         the String type*/

        io::stdin() //handle the input from terminal
            .read_line(&mut guess) //tell where to store the input from terminal, & means that is a reference
            .expect("Failed to read input..."); //this is used to handle possible errors
            /*read_line return a Result variable that can have the value Ok or Err, in case it returns Err will be
              called expect and the program crush*/

        //we transform the guess String into an unsigned 32bit int variable (Shadowing)
        let guess: u32 = match guess //referes to the orignial variable
                                    .trim() //eliminate every whitespace and newline in a String variable
                                    .parse(){ /*transform a string into another type (we specified a 32bit 
                                                unsigned int with ": u32"). It will work just with characters 
                                                that can be logically converted into numbers. 
                                                It return a Result variable as the read_line()*/
                                        Ok(num) => num,
                                        Err(_) => continue, //_ is a catchall value
                                    }; /*We are using a match expression to handle the errors*/

        println!("You guessed: {guess}");

        /*let x = 7;
        let y = 15;
        println!("The sum of {x} and {y} is {}", x + y);*/

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        } /* the match expressions is made up of arms, that are patterns to match against (similar to 
             switch-case in C)*/   
    }
}
