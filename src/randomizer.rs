use std::io;
use rand::prelude::*;

use crate::parser::Card;

pub fn recite(cards: Vec<Card>) {
    let random_number = thread_rng().gen_range(0..cards.len());

    let term = &cards[random_number].term;
    let definition = &cards[random_number].definition;

    println!("What does {:?} mean?", term);
    let mut input = String::new();
    input.clear();
    io::stdin().read_line(&mut input).unwrap();

    if &input.to_lowercase().trim() == definition {
        println!("Well done! {} is indeed {}!", term, definition);
    } else {
        println!("Oh well! {} actually means {}", term, definition)
    }
}