//! This is the game library module.
//! It contains critical functions like get_input(), update_state(), and update_screen()
//! that are crucual for running the game

use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs::read_to_string;
use std::io::stdout;
use std::io::{self, Write};
use std::path::Path;
use std::thread;
use std::time::Duration;

// Indices of all the objects in the game
const LOC_FOREST: usize = 0;
const LOC_DUNGEONS: usize = 1;
const LOC_CAVE: usize = 2;
const LOC_TAVERN: usize = 3;
const LOC_PLAYER: usize = 4;

///Distance enum containing all the distance prompts
#[derive(PartialOrd, Ord, PartialEq, Eq, Debug)]
pub enum Distance {
    Player,
    Held,
    Location,
    Here,
    OverThere,
    NotHere,
    Unknown,
}

/// Command enum containing all the command prompts
pub enum Command {
    Ask(String),
    Drop(String),
    Get(String),
    Attack(String),
    Give(String),
    Look(String),
    Go(String),
    Unknown(String),
    Inventory,
    Quit,
    Help,
    Map,
}

/// Get input from the user
impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Command::Ask(_) => write!(f, "ask"),
            Command::Drop(_) => write!(f, "drop"),
            Command::Get(_) => write!(f, "get"),
            Command::Attack(_) => write!(f, "attack"),
            Command::Give(_) => write!(f, "give"),
            Command::Go(_) => write!(f, "go"),
            Command::Inventory => write!(f, "inventory"),
            Command::Look(_) => write!(f, "look"),
            Command::Quit => write!(f, "quit"),
            Command::Unknown(_) => write!(f, "unknown"),
            Command::Help => write!(f, "help"),
            Command::Map => write!(f, "help"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
/// The object struct
pub struct Object {
    pub label: Vec<String>,
    pub description: String,
    pub location: Option<usize>,
    pub destination: Option<usize>,
    pub item: Option<bool>,
    pub enemy: bool,
    pub health: Option<u64>,
    pub attack: Option<u64>,
}

/// Handles any ambiguous directions
#[derive(PartialOrd, Ord, PartialEq, Eq, Debug)]
pub enum AmbiguousOption<T> {
    None,
    Some(T),
    Ambiguous,
}

#[derive(Serialize, Deserialize, Debug)]
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
                    label: vec!["Forest".to_string()],
                    description: "Look out for tree people".to_string(),
                    location: None,
                    destination: None,
                    item: Some(false),
                    enemy: false,
                    health: None,
                    attack: None,
                },
                Object {
                    label: vec!["Dungeons".to_string()],
                    description: "Be aware of the trolls in the dungeon.".to_string(),
                    location: None,
                    destination: None,
                    item: Some(false),
                    enemy: false,
                    health: None,
                    attack: None,
                },
                Object {
                    label: vec!["Cave".to_string()],
                    description: "Watch out for bats and look for light.".to_string(),
                    location: None,
                    destination: None,
                    item: Some(false),
                    enemy: false,
                    health: None,
                    attack: None,
                },
                Object {
                    label: vec!["Tavern".to_string()],
                    description:
                        "The tavern is empty. But the fire is still burning in the fireplace."
                            .to_string(),
                    location: None,
                    destination: None,
                    item: Some(false),
                    enemy: false,
                    health: None,
                    attack: None,
                },
                Object {
                    label: vec!["Player".to_string()],
                    description: "You".to_string(),
                    location: Some(LOC_FOREST),
                    destination: None,
                    item: Some(false),
                    enemy: false,
                    health: Some(100),
                    attack: None,
                },
                Object {
                    label: vec!["Bear".to_string()],
                    description: "A Grizzly bear".to_string(),
                    location: Some(LOC_CAVE),
                    destination: None,
                    item: Some(false),
                    enemy: true,
                    health: Some(100),
                    attack: Some(10),
                },
                Object {
                    label: vec!["Sword".to_string()],
                    description: "A rusty sword.".to_string(),
                    location: Some(LOC_DUNGEONS),
                    destination: None,
                    item: Some(true),
                    enemy: false,
                    health: None,
                    attack: Some(20),
                },
                Object {
                    label: vec!["Bow".to_string()],
                    description: "A bow.".to_string(),
                    location: Some(LOC_TAVERN),
                    destination: None,
                    item: Some(true),
                    enemy: false,
                    health: None,
                    attack: Some(10),
                },
                Object {
                    label: vec!["Bones".to_string()],
                    description: "Bones of some animal.".to_string(),
                    location: Some(LOC_CAVE),
                    destination: None,
                    item: Some(true),
                    enemy: false,
                    health: None,
                    attack: Some(5),
                },
                Object {
                    label: vec!["North".to_string()],
                    description: "A path to the north leading out of the forest leading to an old Tavern"
                        .to_string(),
                    location: Some(LOC_FOREST),
                    destination: Some(LOC_TAVERN),
                    item: Some(false),
                    enemy: false,
                    health: None,
                    attack: None,
                },
                Object {
                    label: vec!["South".to_string()],
                    description: "A path to the south leading back to the forest".to_string(),
                    location: Some(LOC_TAVERN),
                    destination: Some(LOC_FOREST),
                    item: Some(false),
                    enemy: false,
                    health: None,
                    attack: None,
                },
                Object {
                    label: vec!["East".to_string()],
                    description: "A path to the east leading to the Dungeons".to_string(),
                    location: Some(LOC_TAVERN),
                    destination: Some(LOC_DUNGEONS),
                    item: Some(false),
                    enemy: false,
                    health: None,
                    attack: None,
                },
                Object {
                    label: vec!["West".to_string()],
                    description: "A path to the west leading to the Tavern".to_string(),
                    location: Some(LOC_DUNGEONS),
                    destination: Some(LOC_TAVERN),
                    item: Some(false),
                    enemy: false,
                    health: None,
                    attack: None,
                },
                Object {
                    label: vec!["North".to_string()],
                    description: "A path to the north into a cave".to_string(),
                    location: Some(LOC_DUNGEONS),
                    destination: Some(LOC_CAVE),
                    item: Some(false),
                    enemy: false,
                    health: None,
                    attack: None,
                },
                Object {
                    label: vec!["South".to_string()],
                    description: "A path to the south into the dungeons".to_string(),
                    location: Some(LOC_CAVE),
                    destination: Some(LOC_DUNGEONS),
                    item: Some(false),
                    enemy: false,
                    health: None,
                    attack: None,
                },
                Object {
                    label: vec!["West".to_string(), "East".to_string(), "South".to_string()],
                    description: "You see nothing but trees. There is no other path in that direction."
                        .to_string(),
                    location: Some(LOC_FOREST),
                    destination: None,
                    item: Some(false),
                    enemy: false,
                    health: None,
                    attack: None,
                },
                Object {
                    label: vec!["West".to_string(), "North".to_string()],
                    description: "There is no other path in that direction.".to_string(),
                    location: Some(LOC_TAVERN),
                    destination: None,
                    item: Some(false),
                    enemy: false,
                    health: None,
                    attack: None,
                },
                Object {
                    label: vec!["East".to_string(), "South".to_string()],
                    description:
                        "You see only big rocks and boulders. There is no other path in that direction."
                            .to_string(),
                    location: Some(LOC_DUNGEONS),
                    destination: None,
                    item: Some(false),
                    enemy: false,
                    health: None,
                    attack: None,
                },
                Object {
                    label: vec!["East".to_string(), "North".to_string(), "West".to_string()],
                    description: "The cave has no paths in that direction".to_string(),
                    location: Some(LOC_CAVE),
                    destination: None,
                    item: Some(false),
                    enemy: false,
                    health: None,
                    attack: None,
                },
            ],
        }
    }

    // We are adding reading from file, first step is to read from file.
    pub fn read_from_file(game_file: &str) -> Result<World, std::io::Error> {
        let game_file_path = Path::new(game_file);
        let game_file_data_res = read_to_string(game_file_path);

        match game_file_data_res {
            Ok(game_file_data) => {
                let deserialized_data: Result<World, ron::error::SpannedError> =
                    ron::from_str(&game_file_data);

                match deserialized_data {
                    Ok(deserialized_ron) => Ok(deserialized_ron),
                    Err(de_err_str) => Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        de_err_str.to_string(),
                    )),
                }
            }
            Err(file_err) => Err(file_err),
        }
    }

    /// Check of the game is over
    pub fn game_over(&self) -> bool {
        self.objects[LOC_PLAYER].health == Some(0)
    }

    /// Function for getting the type writer effect
    pub fn type_writer_effect(&self, text: &str) {
        for c in text.chars() {
            print!("{}", c);
            stdout().flush().expect("Could not flush stdout");
            thread::sleep(Duration::from_millis(25));
        }
    }

    /// Check if the object has a label
    fn object_with_label(&self, object: &Object, noun: &str) -> bool {
        let mut result = false;
        for (_, label) in object.label.iter().enumerate() {
            if label.to_lowercase() == noun.to_lowercase() {
                result = true;
                break;
            }
        }
        result
    }

    /// Get the index of the object
    fn object_index(
        &self,
        noun: &str,
        from: Option<usize>,
        max_distance: Distance,
    ) -> AmbiguousOption<usize> {
        let mut result: AmbiguousOption<usize> = AmbiguousOption::None;
        for (position, object) in self.objects.iter().enumerate() {
            if self.object_with_label(object, noun)
                && self.get_distance(from, Some(position)) <= max_distance
            {
                if result == AmbiguousOption::None {
                    result = AmbiguousOption::Some(position);
                } else {
                    result = AmbiguousOption::Ambiguous;
                }
            }
        }
        result
    }

    /// Checks if the object is visible
    fn object_visible(&self, noun: &String) -> (String, Option<usize>) {
        let obj_over_there = self.object_index(noun, Some(LOC_PLAYER), Distance::OverThere);
        let obj_not_here = self.object_index(noun, Some(LOC_PLAYER), Distance::NotHere);

        match (obj_over_there, obj_not_here) {
            // Return none if not a valid command
            (AmbiguousOption::None, AmbiguousOption::None) => {
                ("Invalid command!!".to_string(), None)
            }
            (AmbiguousOption::None, AmbiguousOption::Some(_)) => {
                (format!("You don't see any '{}' here.\n", noun), None)
            }
            // Ambiguous object name
            (AmbiguousOption::Ambiguous, _)
            | (AmbiguousOption::None, AmbiguousOption::Ambiguous) => (
                format!("Please be more specific about which {} you mean.\n", noun),
                None,
            ),
            (AmbiguousOption::Some(index), _) => (String::new(), Some(index)),
        }
    }

    /// Lists all objects in a location
    fn list_objects(&self, location: usize) -> (String, u64) {
        let mut result = String::new();
        let mut count: u64 = 0;
        for (pos, object) in self.objects.iter().enumerate() {
            if pos != LOC_PLAYER
                && self.is_containing(Some(location), Some(pos))
                && object.label.len() == 1
            {
                if count == 0 {
                    result += "\nYou see:\n";
                }
                count += 1;
                result += &format!("{}\n", object.description);
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
            Command::Ask(noun) => self.do_ask(noun),
            Command::Attack(noun) => self.do_attack(noun),
            Command::Drop(noun) => self.do_drop(noun),
            Command::Get(noun) => self.do_get(noun),
            Command::Give(noun) => self.do_give(noun),
            Command::Inventory => self.do_inventory(),
            Command::Help => self.display_help(),
            Command::Map => self.display_locations(),
            Command::Unknown(_) => {
                let invalid_msg = String::from("Invalid command!!\n");
                let help = self.display_help();
                invalid_msg + help.as_str()
            }
        }
    }

    /// Function to perform the attack while attacking an enemy
    pub fn do_use(&mut self, msg: &str, mut obj_health: u64, obj_index: usize) -> u64 {
        let mut split_input = msg.split_whitespace();
        let command = split_input.next().unwrap_or_default().to_string();
        let noun = split_input.next().unwrap_or_default().to_string();
        let list_objects = self.do_inventory();
        if command.contains("inventory") {
            self.type_writer_effect(&list_objects);
            return obj_health;
        }
        let (output, obj_opt) = self.object_visible(&noun);
        match obj_opt {
            Some(weapon_index) => {
                let attack_pwr = self.objects[weapon_index].attack;
                let enemy_pwr = self.objects[obj_index].attack;
                if list_objects.contains(&noun) {
                    obj_health -= attack_pwr.unwrap();
                    self.type_writer_effect(&format!(
                        "You attacked the {}.\nEnemy health: {}",
                        self.objects[obj_index].label[0], obj_health
                    ));
                    if obj_health == 0 {
                        self.objects[obj_index].health = Some(0);
                    }
                    self.type_writer_effect(&format!(
                        "\n\nThe {} attacks",
                        self.objects[obj_index].label[0]
                    ));
                    // random attack
                    let mut rng = rand::thread_rng();
                    let attack: u64 = rng.gen_range(0..enemy_pwr.unwrap());
                    if attack == 0 {
                        self.type_writer_effect("\nYou dodged the attack");
                        obj_health
                    } else {
                        self.type_writer_effect("\nYou got hit");
                        self.objects[LOC_PLAYER].health = Some(
                            self.objects[LOC_PLAYER]
                                .health
                                .map(|h| h - attack)
                                .unwrap_or(0),
                        );
                        self.type_writer_effect(&format!(
                            "\nYour health: {}",
                            self.objects[LOC_PLAYER].health.unwrap_or(0)
                        ));
                        obj_health
                    }
                } else if attack_pwr.is_none() {
                    self.type_writer_effect("That is not a weapon!!");
                    println!("\nHint: Use the following commands: use <weapon name> or run");
                    obj_health
                } else {
                    self.type_writer_effect(&format!("You don't have a {}.\n", noun));
                    obj_health
                }
            }
            None => {
                self.type_writer_effect(&output);
                obj_health
            }
        }
    }

    /// Function to attack an enemy
    pub fn do_attack(&mut self, noun: &String) -> String {
        let (output, obj_opt) = self.object_visible(noun);

        match obj_opt {
            Some(obj_index) => {
                if self.objects[obj_index].enemy {
                    let mut obj_health: u64 =
                        obj_opt.and_then(|a| self.objects[a].health).unwrap_or(0);
                    if obj_health == 0 {
                        return format!(
                            "The {} is already dead.\n",
                            self.objects[obj_index].label[0]
                        );
                    }
                    self.type_writer_effect(&format!(
                        "\nYou are attacking the {}.\n",
                        self.objects[obj_index].label[0]
                    ));
                    println!("\nHint: Use the following commands: use <weapon name> or run");
                    loop {
                        if self.objects[LOC_PLAYER].health.unwrap_or(0) == 0 {
                            return "\nYou died".to_string();
                        }
                        print!("\n> ");
                        io::stdout().flush().unwrap();

                        let mut command = String::new();
                        io::stdin()
                            .read_line(&mut command)
                            .expect("Failed to read input");
                        if command.contains("run") {
                            break;
                        }
                        obj_health = self.do_use(&command, obj_health, obj_index);
                    }
                    if obj_health == 0 {
                        format!("You killed the {}.\n", self.objects[obj_index].label[0])
                    } else {
                        format!(
                            "You ran away from the {}.\n",
                            self.objects[obj_index].label[0]
                        )
                    }
                } else {
                    format!(
                        "You can't attack the {}.\n",
                        self.objects[obj_index].label[0]
                    )
                }
            }
            None => output,
        }
    }

    /// Look around the surroundings of the location the player is in
    pub fn do_look(&self, noun: &str) -> String {
        match noun {
            "" => {
                let (list, _) = self.list_objects(self.objects[LOC_PLAYER].location.unwrap());
                format!(
                    " You are in the {}\n {}.\n",
                    self.objects[self.objects[LOC_PLAYER].location.unwrap()].label[0],
                    self.objects[self.objects[LOC_PLAYER].location.unwrap()].description
                ) + list.as_str()
            }
            _ => "Invalid command!!\n".to_string(),
        }
    }

    /// Player goes to the specified location
    pub fn do_go(&mut self, noun: &String) -> String {
        let (output, obj_opt) = self.object_visible(noun);

        match self.get_distance(Some(LOC_PLAYER), obj_opt) {
            Distance::OverThere => {
                self.objects[LOC_PLAYER].location = obj_opt;
                "OK.\n".to_string() + &self.do_look("")
            }
            Distance::NotHere => {
                format!("You don't see any '{}' here.\n", noun)
            }
            Distance::Unknown => output,
            _ => {
                let obj_dist = obj_opt.and_then(|a| self.objects[a].destination);
                if obj_dist.is_some() {
                    self.objects[LOC_PLAYER].location = obj_dist;
                    "OK.\n".to_string() + &self.do_look("")
                } else {
                    let obj_desc = obj_opt.map(|a| self.objects[a].description.clone());
                    obj_desc.unwrap_or("Invalid command!!\n".to_string())
                }
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
        let (output, obj_opt) = self.object_visible(noun);
        let obj_item = obj_opt.and_then(|a| self.objects[a].item).unwrap_or(false);
        let player_to_obj = self.get_distance(Some(LOC_PLAYER), obj_opt);

        match (player_to_obj, obj_opt, obj_item) {
            (Distance::Player, _, _) => output + "Invalid!! You cannot get that!!",
            (Distance::Held, Some(obj_index), true) => {
                output
                    + &format!(
                        "You already have: {}.\n",
                        self.objects[obj_index].description
                    )
            }
            (Distance::OverThere, _, true) => output + "The item is not here. Try elsewhere!!\n",
            (Distance::OverThere, _, false) => output + "You cannot get that!!\n",
            (Distance::Here, _, false) => output + "You cannot get that!!\n",
            (Distance::Unknown, _, false) => output,
            _ => self.move_object(obj_opt, Some(LOC_PLAYER)),
        }
    }

    /// Player checks the inventory
    pub fn do_inventory(&self) -> String {
        let (list_string, count) = self.list_objects(LOC_PLAYER);
        if count == 0 {
            "You currently do not have anything in your inventory.\n".to_string()
        } else {
            list_string
        }
    }

    /// Returns true or false depending on if the object is contained by another object
    pub fn is_containing(&self, container: Option<usize>, object: Option<usize>) -> bool {
        object.is_some() && (object.and_then(|a| self.objects[a].location) == container)
    }

    /// Returns the distance of one object in relation to another object
    pub fn get_distance(&self, from: Option<usize>, to: Option<usize>) -> Distance {
        let from_loc = from.and_then(|a| self.objects[a].location);

        if to.is_none() {
            Distance::Unknown
        } else if to == from {
            Distance::Player
        } else if self.is_containing(from, to) {
            Distance::Held
        } else if self.is_containing(to, from) {
            Distance::Location
        } else if from_loc.is_some() && self.is_containing(from_loc, to) {
            Distance::Here
        } else if self.passage_index(from_loc, to).is_some() {
            Distance::OverThere
        } else {
            Distance::NotHere
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
                format!("You have dropped {}.\n", self.objects[obj_opt_idx].label[0])
            }
            (Some(obj_opt_idx), _, Some(to_idx), _) if to_idx != LOC_PLAYER => {
                format!(
                    "You put {} in {}.\n",
                    self.objects[obj_opt_idx].label[0], self.objects[to_idx].label[0]
                )
            }
            (Some(obj_opt_idx), Some(obj_loc_idx), _, Some(player_loc_idx))
                if obj_loc_idx == player_loc_idx =>
            {
                format!("You pick up the {}.\n", self.objects[obj_opt_idx].label[0])
            }
            (Some(obj_opt_idx), Some(obj_loc_idx), _, _) => format!(
                "You got {} from {}.\n",
                self.objects[obj_opt_idx].label[0], self.objects[obj_loc_idx].label[0]
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
            (Some(_), None, Some(_)) => "You cannot get that!!\n".to_string(),
            (Some(obj_idx), Some(_), Some(to_idx)) => {
                let output = self.describe_move(obj_opt, to);
                self.objects[obj_idx].location = Some(to_idx);
                output
            }
        }
    }

    /// Gets the index of the passage if visible
    fn passage_index(&self, from: Option<usize>, to: Option<usize>) -> Option<usize> {
        let mut result: Option<usize> = None;

        match (from, to) {
            (Some(from), Some(to)) => {
                for (pos, object) in self.objects.iter().enumerate() {
                    let obj_loc = object.location;
                    let obj_dest = object.destination;
                    match (obj_loc, obj_dest) {
                        (Some(location), Some(destination))
                            if location == from && destination == to =>
                        {
                            result = Some(pos);
                            break;
                        }
                        _ => continue,
                    }
                }
                result
            }
            _ => result,
        }
    }

    /// Returns the index of the object if it is visible
    pub fn get_possession(
        &mut self,
        from: Option<usize>,
        command: Command,
        noun: &String,
    ) -> (String, Option<usize>) {
        let object_held = self.object_index(noun, from, Distance::Held);
        let object_not_here = self.object_index(noun, from, Distance::NotHere);

        match (from, object_held, object_not_here) {
            (None, _, _) => (
                format!("I don't understand what is needed {command}.\n"),
                None,
            ),
            (Some(_), AmbiguousOption::None, AmbiguousOption::None) => (
                format!("Please use correct command for: {}.\n", command),
                None,
            ),
            (Some(from), AmbiguousOption::None, _) if from == LOC_PLAYER => {
                (format!("You are not holding any {}.\n", noun), None)
            }
            (Some(from), AmbiguousOption::Some(object), _) if object == from => (
                format!(
                    "It is illegal to do this: {}.\n",
                    self.objects[object].label[0]
                ),
                None,
            ),
            (Some(_), AmbiguousOption::Ambiguous, _) => (
                format!(
                    "Please be more specific about which {} you want to {}.\n",
                    noun, command
                ),
                None,
            ),
            (Some(_), AmbiguousOption::Some(object_held), _) => ("".to_string(), Some(object_held)),
            (Some(_), AmbiguousOption::None, AmbiguousOption::Some(_))
            | (Some(_), AmbiguousOption::None, AmbiguousOption::Ambiguous) => {
                (format!("You don't see any {} here.\n", noun), None)
            }
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

    pub fn display_help(&self) -> String {
        "Available commands are\n
        look\n
        attack <enemy name>\n
        go <location>\n
        get <item name>\n
        drop <item name>\n
        inventory \n
        quit\n
        help\n"
            .to_string()
    }

    pub fn display_locations(&self) -> String {
        let mut result = String::new();
        result += "Available locations:";
        let mut destinations = std::collections::HashSet::new();

        for object in &self.objects {
            if let Some(destination) = object.destination {
                destinations.insert(destination);
            }
        }

        for (index, object) in self.objects.iter().enumerate() {
            //let location = &self.objects[locations];
            if destinations.contains(&index) {
                //println!("{}: {}", index, object.label[0]);
                result += &format!("{}: {}\n", index, object.label[0]);
            }
        }
        result
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
    let noun = split_input.fold("".to_string(), |accum, item| {
        if accum.is_empty() {
            accum + item
        } else {
            accum + " " + item
        }
    });

    match verb.as_str() {
        "look" => Command::Look(noun),
        "go" => Command::Go(noun),
        "quit" => Command::Quit,
        "ask" => Command::Ask(noun),
        "attack" => Command::Attack(noun),
        "drop" => Command::Drop(noun),
        "get" => Command::Get(noun),
        "give" => Command::Give(noun),
        "help" => Command::Help,
        "inventory" => Command::Inventory,
        "map" => Command::Map,
        _ => Command::Unknown(input.trim().to_string()),
    }
}

/// Function that takes user's input
pub fn get_input() -> Command {
    print!("\n> ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    parse(input)
}

/// Function to update the screen
pub fn update_screen(output: String) {
    for c in output.chars() {
        print!("{}", c);
        stdout().flush().unwrap(); // Flush the output to make it appear immediately
        thread::sleep(Duration::from_millis(25)); // Delay between characters
    }
}
