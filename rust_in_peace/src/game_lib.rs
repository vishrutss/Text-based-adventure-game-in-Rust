//! This is the game library module.
//! It contains critical functions like get_input(), update_state(), and update_screen()
//! that are crucual for running the game

use std::io::{self, Write};

/// Command enum
pub enum Command {
    Look(String),
    Go(String),
    Unkown(String),
    Quit,
}

/// Function that parses user's commands into a verb and a noun
pub fn parse(input: String) -> Command {
    let input = input.to_lowercase();
    let mut split_input = input.split_whitespace();

    let verb = split_input.next().unwrap_or_default().to_string();
    let noun = split_input.next().unwrap_or_default().to_string();

    match verb.as_str() {
        "look" => Command::Look(noun),
        "go" => Command::Go(noun),
        "quit" => Command::Quit,
        _ => Command::Unkown(input.trim().to_string()),
    }
}

/// Function that takes user's input
pub fn get_input() -> Command {
    print!("> ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    parse(input)
}

pub fn update_state(command: &Command) -> String {
    let output: String=match command {
        Command::Look(_)=>{
            "You see nothing but trees all around you. You see a column of smoke rising in the sky. It seems to be very far away.\n".to_string()
        }
        Command::Go(_)=>{
            "You start walking towards the smoke\n".to_string()
        }
        Command::Unkown(_)=>{
            "Invalid command!!\nThese are the available commands: look <add place>, go <add place>, quit\n".to_string()
        }
        Command::Quit => "Quitting.\nThank you for playing!\n".to_string()
    };
    output
}

/// Function to update the screen
pub fn update_screen(output: String) {
    println!("{}", output);
}
