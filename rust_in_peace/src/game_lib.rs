//! This is the game library module.
//! It contains critical functions like get_input(), update_state(), and update_screen()
//! that are crucual for running the game

use std::fmt;
use std::io::{self, Write};
// Indices of all the objects in the game
const LOC_FOREST: usize = 0;
const LOC_DUNGEONS: usize = 1;
const LOC_CAVE: usize = 2;
const LOC_TAVERN: usize = 3;
const LOC_PLAYER: usize = 4;

/// Command enum
pub enum Command {
    Ask(String),
    Drop(String),
    Get(String),
    Give(String),
    Look(String),
    Go(String),
    Unknown(String),
    Inventory,
    Quit,
}

/// Get input from the user
impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Command::Ask(_) => write!(f, "ask"),
            Command::Drop(_) => write!(f, "drop"),
            Command::Get(_) => write!(f, "get"),
            Command::Give(_) => write!(f, "give"),
            Command::Go(_) => write!(f, "go"),
            Command::Inventory => write!(f, "inventory"),
            Command::Look(_) => write!(f, "look"),
            Command::Quit => write!(f, "quit"),
            Command::Unknown(_) => write!(f, "unknown"),
        }
    }
}

/// The object struct
pub struct Object {
    pub name: String,
    pub description: String,
    pub location: Option<usize>,
}

/// The world struct
pub struct World {
    pub objects: Vec<Object>,
}

/// The game struct
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
                    description: "A rusty sword.".to_string(),
                    location: Some(LOC_DUNGEONS),
                },
                Object {
                    name: "Bow".to_string(),
                    description: "A bow.".to_string(),
                    location: Some(LOC_TAVERN),
                },
                Object {
                    name: "Bones".to_string(),
                    description: "Bones of some animal.".to_string(),
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

    /// Check if the object is visible
    fn object_visible(&self, noun: &String) -> (String, Option<usize>) {
        let mut output = String::new();

        let obj_index = self.object_index(noun);
        let obj_loc = obj_index.and_then(|a| self.objects[a].location);
        let obj_container_loc = obj_index
            .and_then(|a| self.objects[a].location)
            .and_then(|b| self.objects[b].location);
        let player_loc = self.objects[LOC_PLAYER].location;

        match (obj_index, obj_loc, obj_container_loc, player_loc) {
            // Return none if not a valid command
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
                output = format!(
                    "Invalid command! Available locations: {}\n\t\t Available objects: {}",
                    location_names, object_names
                );
                (output, None)
            }
            // Object is player
            (Some(obj_index), _, _, _) if obj_index == LOC_PLAYER => (output, Some(obj_index)),
            // Object is the location where the player currently is
            (Some(obj_index), _, _, Some(player_loc)) if obj_index == player_loc => {
                (output, Some(obj_index))
            }
            // Object is held by the player
            (Some(obj_index), Some(obj_loc), _, _) if obj_loc == LOC_PLAYER => {
                (output, Some(obj_index))
            }
            // Object is in the same location as the player
            (Some(obj_index), Some(obj_loc), _, Some(player_loc)) if obj_loc == player_loc => {
                (output, Some(obj_index))
            }
            // Object is a location
            (Some(obj_index), obj_loc, _, _) if obj_loc.is_none() => (output, Some(obj_index)),
            // Invalid object name
            _ => {
                output = format!("You don't see any '{}' here.\n", noun);
                (output, None)
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
                "Please provide the right command. Available commands: \nlook <add place>\ngo <add place>\nget <item name>\ndrop <item name>\nquit\n".to_string()
            }
            Command::Ask(noun) =>self.do_ask(noun),
            Command::Drop(noun) =>self.do_drop(noun),
            Command::Get(noun) => self.do_get(noun),
            Command::Give(noun) => self.do_give(noun),
            Command::Inventory => self.do_inventory(),
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
                "You are at the location\n".to_string()
            }
            (Some(obj_loc), _) => {
                self.objects[LOC_PLAYER].location = Some(obj_loc);
                "OK.\n\n".to_string() + &self.do_look("around")
            }
        }
    }

    /// Player asks the specified object
    pub fn do_ask(&mut self, noun: &String) -> String {
        let player_loc = self.player_here();
        let (output, object_index) =
            self.get_possession(player_loc, Command::Ask("ask".to_string()), noun);
        output + self.move_object(object_index, Some(LOC_PLAYER)).as_str()
    }

    /// Player gives the specified object
    pub fn do_give(&mut self, noun: &String) -> String {
        let player_loc = self.player_here();

        let (output, object_index) =
            self.get_possession(player_loc, Command::Give("give".to_string()), noun);
        output + self.move_object(object_index, Some(LOC_PLAYER)).as_str()
    }

    /// Player drops the specified object
    pub fn do_drop(&mut self, noun: &String) -> String {
        let (output, object_index) =
            self.get_possession(Some(LOC_PLAYER), Command::Drop("drop".to_string()), noun);

        let player_loc = self.objects[LOC_PLAYER].location;
        output + self.move_object(object_index, player_loc).as_str()
    }

    /// Player gets the specified object
    pub fn do_get(&mut self, noun: &String) -> String {
        let (output_vis, obj_opt) = self.object_visible(noun);

        let obj_loc = obj_opt.and_then(|a| self.objects[a].location);

        match (obj_opt, obj_loc) {
            (None, _) => output_vis,
            (Some(object_idx), _) if object_idx == LOC_PLAYER => {
                output_vis + "Please don't do that to yourself.\n"
            }
            (Some(object_idx), Some(obj_loc)) if obj_loc == LOC_PLAYER => {
                output_vis
                    + &format!(
                        "You already have this: {}.\n",
                        self.objects[object_idx].description
                    )
            }
            (obj_opt, _) => self.move_object(obj_opt, Some(LOC_PLAYER)),
        }
    }

    /// Player checks the inventory
    pub fn do_inventory(&self) -> String {
        let (list_string, count) = self.list_objects(LOC_PLAYER);
        if count == 0 {
            //format!("You currently do not have anything in your hands.\n")
            "You currently do not have anything in your hands.\n".to_string()
        } else {
            list_string
        }
    }

    /// Returns the index of the object if it is visible
    pub fn describe_move(&self, obj_opt: Option<usize>, to: Option<usize>) -> String {
        let obj_loc = obj_opt.and_then(|a| self.objects[a].location);
        let player_loc = self.objects[LOC_PLAYER].location;

        match (obj_opt, obj_loc, to, player_loc) {
            (Some(obj_opt_idx), _, Some(to_idx), Some(player_loc_idx))
                if to_idx == player_loc_idx =>
            {
                format!("You have dropped {}.\n", self.objects[obj_opt_idx].name)
            }
            (Some(obj_opt_idx), _, Some(to_idx), _) if to_idx != LOC_PLAYER => {
                format!(
                    "You put {} in {}.\n",
                    self.objects[obj_opt_idx].name, self.objects[to_idx].name
                )
            }
            (Some(obj_opt_idx), Some(obj_loc_idx), _, Some(player_loc_idx))
                if obj_loc_idx == player_loc_idx =>
            {
                format!("You pick up the {}.\n", self.objects[obj_opt_idx].name)
            }
            (Some(obj_opt_idx), Some(obj_loc_idx), _, _) => format!(
                "You got {} from {}.\n",
                self.objects[obj_opt_idx].name, self.objects[obj_loc_idx].name
            ),
            // This arm should never get hit.
            (None, _, _, _) | (_, None, _, _) => "Please you have to drop something.\n".to_string(),
        }
    }

    /// Moves the object to the specified location
    pub fn move_object(&mut self, obj_opt: Option<usize>, to: Option<usize>) -> String {
        let obj_loc = obj_opt.and_then(|a| self.objects[a].location);

        match (obj_opt, obj_loc, to) {
            (None, _, _) => "".to_string(),
            (Some(_), _, None) => "No one is present here to give.\n".to_string(),
            (Some(_), None, Some(_)) => "You have reached your inventory limit!! Please drop something in your inventory before picking it up!!\n".to_string(),
            (Some(obj_idx), Some(_), Some(to_idx)) => {
                let output = self.describe_move(obj_opt, to);
                self.objects[obj_idx].location = Some(to_idx);
                output
            }
        }
    }
    /// Returns the index of the object if it is visible
    pub fn get_possession(
        &mut self,
        from: Option<usize>,
        command: Command,
        noun: &String,
    ) -> (String, Option<usize>) {
        let object_idx = self.object_index(noun);
        let object_loc = object_idx.and_then(|a| self.objects[a].location);

        match (from, object_idx, object_loc) {
            (None, _, _) => (
                format!("I don't understand what is needed {command}.\n"),
                None,
            ),
            (Some(_), None, _) => (
                format!("Please use correct command for: {}.\n", command),
                None,
            ),
            (Some(from_idx), Some(object_idx), _) if object_idx == from_idx => (
                format!(
                    "It is illegal to do this: {}.\n",
                    self.objects[object_idx].name
                ),
                None,
            ),
            (Some(_), Some(object_idx), None) => (
                format!(
                    "It is not possible to do that {}.\n",
                    self.objects[object_idx].name
                ),
                None,
            ),
            (Some(from_idx), Some(object_idx), Some(object_loc_idx))
                if object_loc_idx != from_idx =>
            {
                if from_idx == LOC_PLAYER {
                    (
                        format!(
                            "You are not holding any {}.\n",
                            self.objects[object_idx].name
                        ),
                        None,
                    )
                } else {
                    (
                        format!(
                            "There appears to be no {} you can get from {}.\n",
                            noun, self.objects[from_idx].name
                        ),
                        None,
                    )
                }
            }
            _ => ("".to_string(), object_idx),
        }
    }

    /// Returns player's location
    pub fn player_here(&self) -> Option<usize> {
        let mut player_loc: Option<usize> = None;

        for (pos, object) in self.objects.iter().enumerate() {
            match (pos, object.location) {
                (_, obj_loc) if (obj_loc == self.objects[LOC_PLAYER].location) => {
                    player_loc = Some(pos);
                    break;
                }
                _ => continue,
            }
        }

        player_loc
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
        "ask" => Command::Ask(noun),
        "drop" => Command::Drop(noun),
        "get" => Command::Get(noun),
        "give" => Command::Give(noun),
        "inventory" => Command::Inventory,
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
