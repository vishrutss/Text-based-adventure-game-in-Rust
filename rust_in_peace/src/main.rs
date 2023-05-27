use clearscreen::clear;
use regex::Regex;
use std::io::{stdout, Write};
use std::process::exit;
use std::thread;
use std::time::Duration;
use std::{io, println};

const GAME_FILE_LOCATION: &str = "./game_file.ron";

pub mod game_lib;

fn main() {
    let world_result = init_game(GAME_FILE_LOCATION);

    match world_result {
        Ok(world) => {
            // Here we will run the game
            do_game(world);
        }
        Err(file_err) => {
            println!("Error: {}", file_err);
        }
    }
}
fn init_game(file_location: &str) -> Result<game_lib::World, std::io::Error> {
    //Here we will read the file and return the world we created.

    game_lib::World::read_from_file(file_location)
}

fn do_game(mut world: game_lib::World) {
    clear().expect("Failed to clear screen");
    println!("Hello, Player!\n");
    println!("Welcome to Rust In Peace\n");
    println!("Would you like to start the game? (Y/N)");

    //https://www.geeksforgeeks.org/standard-i-o-in-rust/
    let mut answer = String::new();
    io::stdin()
        .read_line(&mut answer)
        .expect("Failed to read input");

    //https://docs.rs/regex/latest/regex/
    let no = Regex::new("[nN]|[nN][oO]").unwrap();

    //https://doc.rust-lang.org/std/primitive.str.html#method.trim
    if no.is_match(answer.trim()) {
        println!("Goodbye!");
        std::process::exit(0);
    }

    //https://docs.rs/clearscreen/latest/clearscreen/
    clear().expect("Failed to clear screen");

    let message="You find yourself lost in a gloomy forest. You see a column of smoke rising in the sky. It seems to be very far away.\n";
    for c in message.chars() {
        print!("{}", c);
        stdout().flush().unwrap(); // Flush the output to make it appear immediately
        thread::sleep(Duration::from_millis(25)); // Delay between characters
    }

    println!("Hint: Enter <help> to display the commands available");
    let mut command: game_lib::Command;
    //let mut world = game_lib::World::new();
    let mut output: String;

    // Main game loop
    loop {
        if world.game_over() {
            println!("\nWould you like to try again?");
            println!("\n> ");
            io::stdout().flush().unwrap();

            let mut command = String::new();
            io::stdin()
                .read_line(&mut command)
                .expect("Failed to read input");
            if command.trim().to_lowercase() == "y" {
                clear().expect("Failed to clear screen");
                let world_result = init_game(GAME_FILE_LOCATION);

                match world_result {
                    Ok(world) => {
                        // Here we will run the game
                        do_game(world);
                    }
                    Err(file_err) => {
                        println!("Error: {}", file_err);
                        break;
                    }
                }
            } else {
                break;
            }
        }
        command = game_lib::get_input();
        output = world.update_state(&command);
        game_lib::update_screen(output);

        if matches!(command, game_lib::Command::Quit) {
            println!("\nGoodbye!");
            exit(0);
        }
    }

    println!("Goodbye!");
}
