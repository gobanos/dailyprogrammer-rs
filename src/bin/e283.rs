//! [Easy] Anagram Detector
//! Description
//! ===========
//! An anagram is a form of word play, where you take a word (or set of words) and form a different
//! word (or different set of words) that use the same letters, just rearranged. All words must be
//! valid spelling, and shuffling words around doesn't count.
//!
//! Some serious word play aficionados find that some anagrams can contain meaning, like
//! "Clint Eastwood" and "Old West Action", or "silent" and "listen".
//!
//! Someone once said, "All the life's wisdom can be found in anagrams. Anagrams never lie." How
//! they don't lie is beyond me, but there you go.
//!
//! Punctuation, spaces, and capitalization don't matter, just treat the letters as you would
//! scrabble tiles.
//!
//! Input Description
//! =================
//! You'll be given two words or sets of words separated by a question mark. Your task is to replace
//! the question mark with information about the validity of the anagram. Example:
//! "Clint Eastwood" ? "Old West Action"
//! "parliament" ? "partial man"
//!
//! Output Description
//! ==================
//! You should replace the question mark with some marker about the validity of the anagram
//! proposed. Example:
//! "Clint Eastwood" is an anagram of "Old West Action"
//! "parliament" is NOT an anagram of "partial man"
//!
//! Challenge Input
//! ===============
//! "wisdom" ? "mid sow"
//! "Seth Rogan" ? "Gathers No"
//! "Reddit" ? "Eat Dirt"
//! "Schoolmaster" ? "The classroom"
//! "Astronomers" ? "Moon starer"
//! "Vacation Times" ? "I'm Not as Active"
//! "Dormitory" ? "Dirty Rooms"
//!
//! Challenge Output
//! ================
//! "wisdom" is an anagram of "mid sow"
//! "Seth Rogan" is an anagram of "Gathers No"
//! "Reddit" is NOT an anagram of "Eat Dirt"
//! "Schoolmaster" is an anagram of "The classroom"
//! "Astronomers" is NOT an anagram of "Moon starer"
//! "Vacation Times" is an anagram of "I'm Not as Active"
//! "Dormitory" is NOT an anagram of "Dirty Rooms"
//!
//! https://www.reddit.com/r/dailyprogrammer/comments/52enht/20160912_challenge_283_easy_anagram_detector/

#[macro_use] extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::io;

fn is_anagram(a: &str, b: &str) -> bool {
    let mut a = a.to_lowercase().chars().filter(|x| x.is_alphabetic()).collect::<Vec<_>>();
    let mut b = b.to_lowercase().chars().filter(|x| x.is_alphabetic()).collect::<Vec<_>>();

    a.sort();
    b.sort();

    a == b
}

fn main() {
    loop {
        match parse_input() {
            Some((a, b)) => {
                if is_anagram(&a, &b) {
                    println!("\"{}\" is an anagram of \"{}\"", a, b);
                } else {
                    println!("\"{}\" is NOT an anagram of \"{}\"", a, b);
                }
            },
            None => break
        }
    }
}

fn parse_input() -> Option<(String, String)> {
    lazy_static! {
        static ref RE: Regex = Regex::new("^\"(.*)\" \\? \"(.*)\"$").unwrap();
    }
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let input = input.trim();

    match RE.captures(input) {
        Some(caps) => {
            Some((
                caps.at(1).unwrap().to_string(),
                caps.at(2).unwrap().to_string(),
            ))
        },
        None => None
    }
}

#[test]
fn challenge() {
    assert!(is_anagram("wisdom", "mid sow"));
    assert!(is_anagram("Seth Rogan", "Gathers No"));
    assert!(!is_anagram("Reddit", "Eat Dirt"));
    assert!(is_anagram("Schoolmaster", "The classroom"));
    assert!(!is_anagram("Astronomers", "Moon starer"));
    assert!(is_anagram("Vacation Times", "I'm Not as Active"));
    assert!(!is_anagram("Dormitory", "Dirty Rooms"));
}