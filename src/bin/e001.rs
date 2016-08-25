//! create a program that will ask the users name, age, and reddit username. have it tell them the information back, in the format:
//!
//! your name is (blank), you are (blank) years old, and your username is (blank)
//!
//! for extra credit, have the program log this information in a file to be accessed later.
//! https://www.reddit.com/r/dailyprogrammer/comments/pih8x/easy_challenge_1/

use std::io::{ self, Write };
use std::fs;

fn main() {
    // Ask the name
    println!("What's your name ?");
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();

    // Ask the age
    println!("What's your age ?");
    let mut age = String::new();
    io::stdin().read_line(&mut age).unwrap();

    // Ask the username
    println!("What's your reddit username ?");
    let mut username = String::new();
    io::stdin().read_line(&mut username).unwrap();

    // Create the file `infos.txt`
    let mut file = fs::File::create("infos.txt").unwrap();

    // Make the info string and print it to `infos.txt`
    write!(
        file,
        "your name is {}, you are {} years old, and your username is {}",
        name.trim(),
        age.trim(),
        username.trim()
    ).unwrap();
}