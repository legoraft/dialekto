use std::io;

use crate::parser::Card;

pub fn recite(index: usize, cards: &Vec<Card>) {
    let term = &cards[index].term;
    let definition = &cards[index].definition;

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