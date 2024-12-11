use rand::seq::SliceRandom;
use std::io::{self, Write};

fn main() {
    println!("Welcome to rust randomizer");
    println!("Write or paste your list of options below and press Enter twice when done:");

    let mut input = String::new();

    // Read multiline input
    loop {
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read input");
        
        if line.trim().is_empty() {
            break; // Stop reading when an empty line is entered
        }
        
        input.push_str(&line);
    }

    // Split the input into lines, trim whitespace, and collect non-empty lines
    let options: Vec<String> = input
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect();

    if options.is_empty() {
        println!("No options were provided. Exiting.");
        return;
    }

    // Ask the user how many options to select
    let selected_count = loop {
        print!("How many options would you like to select? ");
        io::stdout().flush().unwrap(); // Ensure the prompt is displayed immediately

        let mut count_input = String::new();
        io::stdin()
            .read_line(&mut count_input)
            .expect("Failed to read input");

        match count_input.trim().parse::<usize>() {
            Ok(n) if n > 0 && n <= options.len() => break n,
            _ => println!("Please enter a valid number between 1 and {}.", options.len()),
        }
    };

    // Randomly select the specified number of unique options
    let mut rng = rand::thread_rng();
    let selected_options: Vec<_> = options
        .choose_multiple(&mut rng, selected_count)
        .cloned()
        .collect();

    println!("\nRandomly selected options:");
    for option in selected_options {
        println!("- {}", option);
    }
}
