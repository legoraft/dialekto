use std::io;
use rand::prelude::*;

use crate::parser::Card;

pub fn recite(cards: &Vec<Card>) {
    let next_card = thread_rng().gen_range(0..cards.len());

    let term = &cards[next_card].term;
    let definition = &cards[next_card].definition;

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