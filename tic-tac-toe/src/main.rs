use std::io;
use std::process;
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut player: char = 'X';
    let mut game_table = Vec::new();
    init_game_table(&mut game_table);

    //game loop
    loop {
        process::Command::new("clear").status().unwrap(); //clear screen

        print_game_table(&game_table);
        let cell_number = select_cell(player) - 1; //decreased by 1 beacuse the player will see cells that starts from 1, but the index of the first element of the vector is 0, not 1

        //check if the cell was selected before
        if game_table[cell_number] == 'X' || game_table[cell_number] == 'O' {
            continue;
        } else {
            game_table[cell_number] = player;
            //check if someone win
            if check_win(player, &game_table) == 'a' {
                // change player
                if player == 'X' {
                    player = 'O';
                } else {
                    player = 'X';
                }
            } else {
                process::Command::new("clear").status().unwrap(); //clear screen
                print_game_table(&game_table);
                println!("Player {} WIN!!!", player);
                break;
            }
        }
    }
    ExitCode::SUCCESS
}



fn init_game_table(gt: &mut Vec<char>) {
    let mut i: u32 = 1;
    while i < 10 {
        let c = char::from_digit(i, 10);
        gt.push(c.unwrap());
        i += 1;
    }
}

fn print_game_table(gt: &Vec<char>) {
    let mut i = 0;
    let mut index = 0;
    while i < 3 {
        println!("+---+---+---+");
        let mut j = 0;
        while j < 3 {
            print!("| {} ", gt[index]);
            j += 1;
            index += 1;
        }
        println!("|");
        i += 1;
    }
    println!("+---+---+---+");
}

fn select_cell(player: char) -> usize {
    let value: usize;
    loop {
        println!("Turn of player {}", player);
        println!("Cell number: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Something went wrong while reading input");
        value = match input.trim().parse() {
            Ok(num) => {
                if num > 9 || num < 1 {
                    println!("ERROR: you must tipe a number within 1 and 9");
                    continue;
                } else {
                    num
                }
            },
            Err(_) => {
                println!("ERROR: you must tipe a number within 1 and 9");
                continue;
            }
        };
        break;
    }
    return value;
}

fn check_win(player: char ,v: &Vec<char>) -> char {
    if (v[0] == v[3] && v[0] == v[6]) || (v[1] == v[4] && v[1] == v[7]) || (v[2] == v[5] && v[2] == v[8]) {
        return player; //horizontal
    } else if (v[0] == v[1] && v[0] == v[2]) || (v[3] == v[4] && v[3] == v[5]) || (v[6] == v[7] && v[6] == v[8]) {
        return player; //vertical
    } else if (v[0] == v[4] && v[0] == v[8]) || (v[2] == v[4] && v[2] == v[6]) {
        return player; //diagonal
    } else {
        return 'a'; //nobody won
    }
}