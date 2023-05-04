//! This is the game library module.
//! It contains critical functions like get_input(), update_state(), and update_screen()
//! that are crucual for running the game

use std::io::{self, Write};

// Indices of all the objects in the game
const LOC_FOREST: usize = 0;
const LOC_DUNGEONS: usize = 1;
const LOC_CAVE: usize = 2;
const LOC_TAVERN: usize = 3;
const LOC_PLAYER: usize = 4;

/// Command enum
pub enum Command {
    Look(String),
    Go(String),
    Unknown(String),
    Quit,
}
pub struct Object {
    pub name: String,
    pub description: String,
    pub location: Option<usize>,
}

pub struct World {
    pub objects: Vec<Object>,
}

impl World {
    pub fn new() -> Self {
        World {
            objects: vec![
                Object {
                    name: "Forest".to_string(),
                    description: "Look out for tree people".to_string(),
                    location: None,
                },
                Object {
                    name: "Dungeons".to_string(),
                    description: "Be aware of the trolls in the dungeon.".to_string(),
                    location: None,
                },
                Object {
                    name: "Cave".to_string(),
                    description: "Watch out for bats and look for light.".to_string(),
                    location: None,
                },
                Object {
                    name: "Tavern".to_string(),
                    description:
                        "The tavern is empty. But the fire is still burning in the fireplace."
                            .to_string(),
                    location: None,
                },
                Object {
                    name: "Player".to_string(),
                    description: "You".to_string(),
                    location: Some(LOC_FOREST),
                },
                Object {
                    name: "Sword".to_string(),
                    description: "A rusty sword stuck in the wall.".to_string(),
                    location: Some(LOC_DUNGEONS),
                },
                Object {
                    name: "Bow".to_string(),
                    description: "A bow on the ground.".to_string(),
                    location: Some(LOC_TAVERN),
                },
                Object {
                    name: "Bones".to_string(),
                    description: "There are bones of some animal on the ground!!".to_string(),
                    location: Some(LOC_CAVE),
                },
            ],
        }
    }

    /// Check if the object has a name
    fn object_with_name(&self, object: &Object, noun: &String) -> bool {
        *noun == object.name.to_lowercase()
    }

    /// Get the index of the object
    fn object_index(&self, noun: &String) -> Option<usize> {
        let mut result: Option<usize> = None;
        for (position, object) in self.objects.iter().enumerate() {
            if self.object_with_name(object, noun) {
                result = Some(position);
                break;
            }
        }
        result
    }

    /// Gets the objests that are visible to the player at a given location
    fn object_visible(&self, noun: &String) -> (String, Option<usize>) {
        let mut result = String::new();
        let index = self.object_index(noun);
        let obj_location = index.and_then(|a| self.objects[a].location);
        let obj_container_loc = index
            .and_then(|a| self.objects[a].location)
            .and_then(|b| self.objects[b].location);
        let player_location = self.objects[LOC_PLAYER].location;
        match (index, obj_location, obj_container_loc, player_location) {
            // Return none of not a valid command
            (None, _, _, _) => {
                //https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect
                //https://doc.rust-lang.org/std/iter/struct.Map.html?search=collect
                //https://doc.rust-lang.org/alloc/slice/trait.Join.html
                /*
                The line of code below uses iter() method to iterate over each location in world.locations,
                then uses filter() to select only objects that are locations,
                then uses map() method to create a new iterator that clones each location’s name,
                and finally uses collect() method to collect all the cloned names into a vector and join them
                with commas using join() method.
                */
                let location_names: String = self
                    .objects
                    .iter()
                    .filter(|object| object.location.is_none())
                    .map(|object| object.name.clone())
                    .collect::<Vec<_>>()
                    .join(", ");
                let object_names: String = self
                    .objects
                    .iter()
                    .filter(|object| object.location.is_some())
                    .map(|object| object.name.clone())
                    .collect::<Vec<_>>()
                    .join(", ");
                result = format!(
                    "Invalid! Available locations: {}\n\tAvailable objects: {}",
                    location_names, object_names
                );
                (result, None)
            }
            // Object is player
            (Some(index), _, _, _) if index == LOC_PLAYER => (result, Some(index)),

            // Object is the location where the player currently is
            (Some(index), _, _, Some(player_location)) if index == player_location => {
                (result, Some(index))
            }
            // Object is held by the player
            (Some(index), Some(obj_location), _, _) if obj_location == LOC_PLAYER => {
                (result, Some(index))
            }
            // Object is in the same location as the player
            (Some(index), Some(obj_location), _, Some(player_location))
                if obj_location == player_location =>
            {
                (result, Some(index))
            }
            // Object is a location
            (Some(index), obj_location, _, _) if obj_location.is_none() => (result, Some(index)),

            // Invalid object name
            _ => {
                result = format!("You don't see any '{}' here.\n", noun);
                (result, None)
            }
        }
    }

    /// Lists all objects in a location
    fn list_objects(&self, location: usize) -> (String, u64) {
        let mut result = String::new();
        let mut count: u64 = 0;
        for (pos, object) in self.objects.iter().enumerate() {
            match (pos, object.location) {
                (pos, _) if pos == LOC_PLAYER => continue,
                (_, None) => continue,
                (_, Some(obj_location)) if obj_location == location => {
                    if count == 0 {
                        result += "You see:\n";
                    }
                    count += 1;
                    result = result + &format!("{}\n", object.description);
                }
                _ => continue,
            }
        }
        (result, count)
    }

    /// Updates state of the game
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

    /// Look around the surroundings of the location the player is in
    pub fn do_look(&self, noun: &str) -> String {
        match noun {
            "around" | "" => {
                let (list, _) = self.list_objects(self.objects[LOC_PLAYER].location.unwrap());
                format!(
                    " You are in the {}\n {}.\n",
                    self.objects[self.objects[LOC_PLAYER].location.unwrap()].name,
                    self.objects[self.objects[LOC_PLAYER].location.unwrap()].description
                ) + list.as_str()
            }
            _ => "Invalid command!!\n".to_string(),
        }
    }

    /// Player goes to the specified location
    pub fn do_go(&mut self, noun: &String) -> String {
        let (output, obj_opt) = self.object_visible(noun);
        let player_loc = self.objects[LOC_PLAYER].location;
        match (obj_opt, player_loc) {
            (None, _) => output,
            (Some(obj_loc), Some(player_loc)) if obj_loc == player_loc => {
                "You are looking at yourself\n".to_string()
            }
            (Some(obj_loc), _) => {
                self.objects[LOC_PLAYER].location = Some(obj_loc);
                "OK.\n\n".to_string() + &self.do_look("around")
            }
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
