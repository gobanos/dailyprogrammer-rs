//! create a program that will allow you to enter events organizable by hour.
//! There must be menu options of some form, and you must be able to easily edit, add,
//! and delete events without directly changing the source code.
//!
//! (note that by menu i dont necessarily mean gui. as long as you can easily access the different
//! options and receive prompts and instructions telling you how to use the program,
//! it will probably be fine)
//! https://www.reddit.com/r/dailyprogrammer/comments/pihtx/intermediate_challenge_1/

use std::io;

/// A day represent a per hour planning
#[derive(Debug)]
struct Day {
    planning: Vec<Option<Event>>,
}

impl Day {
    fn new() -> Day {
        let planning = (0..24).map(|_| None).collect::<Vec<_>>();
        Day {
            planning: planning
        }
    }
}

type Hour = usize;
type Event = String;

#[derive(Debug)]
enum Action {
    Add(Hour, Event),
    Remove(Hour),
    Quit,
}

fn main() {
    let mut day = Day::new();

    // loop until the user ask to quit
    loop {
        // print the planning
        println!("{:?}", day);

        // handle the user action
        match get_action() {
            Action::Add(hour, event) => {
                day.planning[hour] = Some(event);
            },
            Action::Remove(hour) => {
                day.planning[hour] = None;
            },
            Action::Quit => {
                break;
            }
        }
    }

    println!("Have a good day !");
}

fn get_action() -> Action {
    println!("What do you want to do ?");
    println!("\t1. Add an event");
    println!("\t2. Remove an event");
    println!("\tQ. Quit");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    match input.trim() {
        // Add an event
        "1" => {
            // Ask the hour
            println!("Enter the hour :");
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).unwrap();
            let hour = buffer.trim().parse().unwrap();

            // Assert the hour is valid
            if hour >= 24 {
                panic!("Hour should be contain between 0 and 23");
            }

            // Ask the event name
            println!("Enter the event name :");
            let mut name = String::new();
            io::stdin().read_line(&mut name).unwrap();

            let event = name.trim().to_string();

            Action::Add(hour, event)
        },
        // Remove an event
        "2" => {
            // Ask the hour
            println!("Enter the hour :");
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).unwrap();
            let hour = buffer.trim().parse().unwrap();

            // Assert the hour is valid
            if hour >= 24 {
                panic!("Hour should be contain between 0 and 23");
            }

            Action::Remove(hour)
        },
        // Quit the program
        "Q" => {
            Action::Quit
        },
        _ => {
            panic!("Invalid action");
        }
    }
}