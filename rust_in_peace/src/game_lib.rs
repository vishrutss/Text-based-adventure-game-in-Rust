//! This is the game library module.
//! It contains critical functions like get_input(), update_state(), and update_screen()
//! that are crucual for running the game

/// Command structure
pub enum Command {
    Look(String),
    Go(String),
    Unkown(String),
    Quit,
}

pub fn parse(input: String) -> Command {
    let input = input.to_lowercase();
    let mut split_input = input.trim().split_whitespace();

    let verb = split_input.next().unwrap_or_default().to_string();
    let noun = split_input.next().unwrap_or_default().to_string();

    match verb.as_str() {
        "look" => Command::Look(noun),
        "go" => Command::Go(noun),
        "quit" => Command::Quit,
        _ => Command::Unkown(input.trim().to_string()),
    }
}
