use std::{env, fs};
use parser::parse_cards;
use rand::prelude::*;

mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("No file added!");
    }

    let cards_txt = fs::read_to_string(&args[1])
        .expect("Unable to read file.");

    let cards = parse_cards(cards_txt);
    let random_number = thread_rng().gen_range(0..=cards.len());

    println!("{} is {}!", cards[random_number].term, cards[random_number].definition);
}
