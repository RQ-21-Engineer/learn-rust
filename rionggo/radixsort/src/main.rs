use std::env;
use std::fs::File;
use std::io::{self, BufReader, Read};

fn main() {
// Display menu for user input or file input
    println!("Welcome to the radix sort program!");
    println!("Please select an input method:");
    println!("1. User input");
    println!("2. File input");
    println!("Your choice: ");
    // Read user selection
    let mut input_method = String::new();
    io::stdin().read_line(&mut input_method).unwrap();

    let input_method: u32 = input_method.trim().parse().unwrap();

// Read in numbers based on user selection
    let mut numbers: Vec<u32> = Vec::new();
    match input_method {
        1 => {
            println!("Enter numbers separated by spaces: ");

            let mut user_input = String::new();
            io::stdin().read_line(&mut user_input).unwrap();

            numbers = user_input
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
        }
        2 => {
            println!("Enter the file path: ");

            let mut file_path = String::new();
            io::stdin().read_line(&mut file_path).unwrap();

            let file = File::open(file_path.trim()).unwrap();
            let mut reader = BufReader::new(file);
            let mut file_contents = String::new();
            reader.read_to_string(&mut file_contents).unwrap();

            numbers = file_contents
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
        }
        _ => {
            println!("Invalid input method selected");
            return;
        }
    }

// Perform radix sort on numbers
    let max_digits = get_max_digits(&numbers);
    for i in 0..max_digits {
        let mut buckets: Vec<Vec<u32>> = vec![Vec::new(); 10];

        for number in &numbers {
            let digit = get_digit(*number, i);
            buckets[digit as usize].push(*number);
        }

        numbers = buckets.into_iter().flatten().collect();
    }

// Print sorted numbers
    println!("Sorted numbers: {:?}", numbers);
}

fn get_max_digits(numbers: &Vec<u32>) -> u32 {
    let mut max_digits = 0;
    for number in numbers {
        let digits = get_num_digits(*number);
        if digits > max_digits {
            max_digits = digits;
        }
    }
    max_digits
}

fn get_num_digits(mut number: u32) -> u32 {
    let mut digits = 0;
    while number > 0 {
        number /= 10;
        digits += 1;
    }
    digits
}

fn get_digit(mut number: u32, i: u32) -> u32 {
    for _ in 0..i {
        number /= 10;
    }
    number % 10
}