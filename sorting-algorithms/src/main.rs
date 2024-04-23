use rand::{Rng, thread_rng};
use std::process::ExitCode;
use std::env; //for command line arguments
/**
 * arg1 = sorting algorithm: 
 *          b: bubble sort
 *          rb: revised bubble sort
 *          s: selection sort
 */

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Error: you must specify an argument.");
        ExitCode::FAILURE
    } else {
        match args[1].as_str() {
            "b" | "rb" | "s" => { 
                let mut array = init_array();
                /* println!("Random number array");
                println!("{:?}", array); */
                if args[1] == "b" {
                    bubble_sort(&mut array);
                } else if args[1] == "rb" {
                    revised_bubble_sort(&mut array);
                } else {
                    selection_sort(&mut array);
                }
                println!("\nSorted array");
                println!("{:?}", array);
                ExitCode::SUCCESS 
            },
            _ => { 
                println!("Error: invalid argument.");
                println!("Accepted arguments:");
                println!("\tb: to use Bubble Sort"); 
                println!("\trb: to use Revised Bubble Sort"); 
                println!("\ts: to use Selection Sort"); 
                ExitCode::FAILURE 
            },
        }
    }
}



fn init_array() -> [u32; 10000] {
    let mut rng = thread_rng(); //set the seed for random numbers
    let mut array: [u32; 10000] = [0; 10000]; //create a mutable array of 10000 u32 elemets initialized to 0
    //init array with random numbers in range from 0 to 1000;
    let mut i = 0;
    while i < 10000 {
        array[i] = rng.gen_range(0..10000);
        i += 1;
    }
    array
}

fn bubble_sort(arr: &mut [u32; 10000]) {
    let len = arr.len();
    for i in 0..len-1 {
        for j in 0..len-i-1{
            if arr[j] > arr[j+1] {
                arr.swap(j, j+1);
            }
        }
    }
}

fn revised_bubble_sort(arr: &mut [u32; 10000]) {
    let mut is_swapped = false;
    let len = arr.len();
    for i in 0..len-1 {
        for j in 0..len-i-1{
            if arr[j] > arr[j+1] {
                arr.swap(j, j+1);
                is_swapped = true;
            }
        }
        if !is_swapped {
            break;
        }
    }
}

fn selection_sort(arr: &mut [u32; 10000]) {
    let len = arr.len();
    for i in 0..len-1 {
        let mut min_index = i;
        for j in (i+1)..len {
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }
        arr.swap(i, min_index);
    }
}