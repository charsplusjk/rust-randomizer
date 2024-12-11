use rand::Rng;
use std::io::{self, Write};

fn main() {
    println!("\nWelcome to rust randomizer!");
    println!("Enter your options one by one. Type 'done' when you're finished:");

    let mut inputs = Vec::new();

    loop {
        print!("Enter an option: ");
        io::stdout().flush().unwrap(); // Ensure the prompt is displayed immediately

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let input = input.trim(); // Remove any surrounding whitespace

        if input.eq_ignore_ascii_case("done") {
            break;
        }

        if !input.is_empty() {
            inputs.push(input.to_string());
        }
    }

    if inputs.is_empty() {
        println!("No options were provided. Exiting.");
        return;
    }

    // Randomly select one of the inputs
    let random_index = rand::thread_rng().gen_range(0..inputs.len());
    let selected = &inputs[random_index];

    println!("\nSelected option: {}", selected);
}
