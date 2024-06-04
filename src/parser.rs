use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash, Debug)]
struct Card {
    term: String,
    definition: String,
}

fn parse_cards(cards_txt: &str) -> HashSet<Card> {
    let lines: Vec<&str> = cards_txt.trim().lines().collect();
    let mut cards = HashSet::new();
    
    for line in lines {
        let (term, definition) = line.split_once("; ").unwrap();
        let term = term.to_string();
        let definition = definition.to_string();
        
        cards.insert(Card{
            term,
            definition
        });
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
        let cards = HashSet::from([Card{
            term: "hello".to_string(), definition: "bonjour".to_string()
        },
        Card{
            term: "goodbye".to_string(), definition: "au revoir".to_string()
        }]);

        assert_eq!(cards, parse_cards(text));
    }
}
