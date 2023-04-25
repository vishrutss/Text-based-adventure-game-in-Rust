use std::io;
use regex::Regex;

fn main() {
    println!("Hello, Player!");
    println!("");
    println!("Welcome to Rust In Peace");
    println!("");
    println!("Would you like to start the game? (Y/N)");
    
    //https://www.geeksforgeeks.org/standard-i-o-in-rust/
    let mut answer=String::new();
    io::stdin().read_line(&mut answer).expect("failed to read input");
    
    //https://docs.rs/regex/latest/regex/
    let no = Regex::new("[nN]|[nN][oO]+").unwrap();

    //https://doc.rust-lang.org/std/primitive.str.html#method.trim
    if no.is_match(answer.trim()){
        println!("Goodbye!");
        std::process::exit(1);
    }
}
