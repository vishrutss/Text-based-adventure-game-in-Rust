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
                    name: "Forest".to_string(),
                    description: "Look out for tree people".to_string(),
                },
                Location {
                    name: "Dungeons".to_string(),
                    description: "Be aware of the trolls in the dungeon".to_string(),
                },
                Location {
                    name: "Cave".to_string(),
                    description: "Watch out for bats and look for light".to_string(),
                },
            ],
        }
    }

    pub fn update_state(&mut self, command: &Command) -> String {
        match command {
            Command::Look(noun) => self.do_look(noun),
            Command::Go(noun) => self.do_go(noun),
            Command::Quit => "Quitting.\nThank you for playing!".to_string(),
            Command::Unknown(_) => {
                "Please provide the right command. Available commands: look <add place>, go <add place>, quit\n".to_string()
            }
        }
    }

    pub fn do_look(&self, noun: &str) -> String {
        match noun {
            "around" | "" => format!(
                " You are in the {}\n {}.\n",
                self.locations[self.player_location].name,
                self.locations[self.player_location].description
            ),
            _ => "Seek for the right path.\n".to_string(),
        }
    }

    pub fn do_go(&mut self, noun: &String) -> String {
        let mut output = String::new();

        for (pos, location) in self.locations.iter().enumerate() {
            if *noun == location.name.to_lowercase() {
                if pos == self.player_location {
                    output += "Wherever you go, there you are.\n";
                } else {
                    self.player_location = pos;
                    output = output + "OK.\n\n" + &self.do_look("around");
                }
                break;
            }
        }

        if output.is_empty() {
            //https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect
            //https://doc.rust-lang.org/std/iter/struct.Map.html?search=collect
            //https://doc.rust-lang.org/alloc/slice/trait.Join.html
            /*
            The line of code below uses iter() method to iterate over each location in world.locations,
            then uses map() method to create a new iterator that clones each location’s name,
            and finally uses collect() method to collect all the cloned names into a vector and join them
            with commas using join() method.
            */
            let location_names = self
                .locations
                .iter()
                .map(|location| location.name.clone())
                .collect::<Vec<_>>()
                .join(", ");
            format!(
                "I don't understand where you want to go. Availabe locations: {}",
                location_names
            )
        } else {
            output
        }
    }
}

/// Default implementation for World
impl Default for World {
    fn default() -> Self {
        Self::new()
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

/// Function to update the screen
pub fn update_screen(output: String) {
    println!("{}", output);
}
