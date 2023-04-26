use clearscreen::clear;
use regex::Regex;
use std::io;

pub mod game_lib;

fn main() {
    println!("Hello, Player!\n");
    println!("Welcome to Rust In Peace\n");
    println!("Would you like to start the game? (Y/N)");

    //https://www.geeksforgeeks.org/standard-i-o-in-rust/
    let mut answer = String::new();
    io::stdin()
        .read_line(&mut answer)
        .expect("failed to read input");

    //https://docs.rs/regex/latest/regex/
    let no = Regex::new("[nN]|[nN][oO]").unwrap();

    //https://doc.rust-lang.org/std/primitive.str.html#method.trim
    if no.is_match(answer.trim()) {
        println!("Goodbye!");
        std::process::exit(0);
    }

    //https://docs.rs/clearscreen/latest/clearscreen/
    clear().expect("failed to clear screen");

    println!("You find yourself in a gloomy forest.")
}
