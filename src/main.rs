use std::{env, fs::{self, read_to_string}};

use parser::parse_cards;

mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("No file added!");
    }

    let cards_txt = fs::read_to_string(&args[1])
        .expect("Unable to read file.");

    let cards = parse_cards(cards_txt);
}
