use std::io::{self, Write};
use colored::*;

fn main() {
    loop {
        println!("{}", "========================".bright_blue());
        println!("{}", "      RustyPixels       ".bold().green());
        println!("{}", "========================".bright_blue());
        println!("{}", "1. ".yellow().bold().to_string() + "Image to ASCII");
        println!("{}", "2. ".yellow().bold().to_string() + "Figlet (Text to ASCII)");
        println!("{}", "0. ".yellow().bold().to_string() + "Exit");
        print!("{}", "Enter your choice: ".italic().cyan());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => println!("{}", "Launching Image to ASCII...".green()),
            "2" => println!("{}", "Launching Figlet...".magenta()),
            "0" => {
                println!("{}", "Goodbye!".bold().red());
                break;
            }
            _ => println!("{}", "Invalid option, try again!".red()),
        }

        println!();
    }
}

