use std::{env, fs};
use parser::{parse_cards, Card};
use randomizer::generate_indices;
use reciter::recite;

mod parser;
mod randomizer;
mod reciter;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("No file added!");
    }

    let cards_txt = fs::read_to_string(&args[1])
        .expect("Unable to read file.");

    let cards: Vec<Card> = parse_cards(cards_txt);
    let indices: Vec<usize> = generate_indices(cards.len());

    for index in indices {
        recite(index, &cards);
    }
}
