//! This is the game library module.
//! It contains critical functions like get_input(), update_state(), and update_screen()
//! that are crucual for running the game

use std::io::{self, Write};

/// Command enum
pub enum Command {
    Look(String),
    Go(String),
    Unknown(String),
    Quit,
}
pub struct Location {
    pub name: String,
    pub description: String,
}

pub struct World {
    pub player_location: usize,
    pub locations: Vec<Location>,
}

impl World {
    pub fn new() -> Self {
        World {
            player_location: 0,
            locations: vec![
                Location {
                    name: "Dungeons".to_string(),
                    description: "Be aware of the trolls in the dungeon".to_string(),
                },
                Location {
                    name: "Cave".to_string(),
                    description: "Watch out for bats and look for light".to_string(),
                },
                Location {
                    name: "Forest".to_string(),
                    description: "Look out for tree people".to_string(),
                },
            ],
        }
    }

    pub fn update_state(&mut self, command: &Command) -> String {
        match command {
            Command::Look(noun) => self.do_look(noun),
            Command::Go(noun) => self.do_go(noun),
            Command::Quit => format!("Quitting.\nThank you for playing!"),
            Command::Unknown(input_str) => {
                format!("Please provide the right command {}'.", input_str)
            }
        }
    }

    pub fn do_look(&self, noun: &String) -> String {
        match noun.as_str() {
            "around" | "" => format!(
                " Welcome to the {}\n {}.\n",
                self.locations[self.player_location].name,
                self.locations[self.player_location].description
            ),
            _ => format!("Seek for the right path.\n"),
        }
    }

    pub fn do_go(&mut self, noun: &String) -> String {
        let mut output = String::new();

        for (pos, location) in self.locations.iter().enumerate() {
            if *noun == location.name.to_lowercase() {
                if pos == self.player_location {
                    output = output + &format!("Wherever you go, there you are.\n");
                } else {
                    self.player_location = pos;
                    output = output + &format!("OK.\n\n") + &self.do_look(&"around".to_string());
                }
                break;
            }
        }

        if output.len() == 0 {
            format!("I don't understand where you want to go.")
        } else {
            output
        }
    }
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
        _ => Command::Unknown(input.trim().to_string()),
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

/*
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
*/

/// Function to update the screen
pub fn update_screen(output: String) {
    println!("{}", output);
}
