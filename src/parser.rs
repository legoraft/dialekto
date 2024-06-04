use std::collections::HashMap;

fn parse_cards(cards_txt: &str) -> HashMap<&str, &str> {
    let lines: Vec<&str> = cards_txt.trim().lines().collect();
    let mut cards: HashMap<&str, &str> = HashMap::new();
    
    for line in lines {
        let (word, definition) = line.split_once("; ").unwrap();
        cards.insert(word, definition);
    }

    cards
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn card_parser() {
        let text = "
hello; bonjour
goodbye; au revoir
";
        let cards = HashMap::from([("hello", "bonjour"), ("goodbye", "au revoir")]);

        assert_eq!(cards, parse_cards(text));
    }
}
