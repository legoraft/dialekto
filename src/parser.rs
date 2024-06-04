#[derive(PartialEq, Debug)]
struct Card {
    term: String,
    definition: String,
}

fn parse_cards(cards_txt: &str) -> Vec<Card> {
    let lines: Vec<&str> = cards_txt.trim().lines().collect();
    let mut cards: Vec<Card> = Vec::new();
    
    for line in lines {
        let (term, definition) = line.split_once("; ").unwrap();
        let term = term.to_string();
        let definition = definition.to_string();
        
        cards.push(Card{
            term,
            definition,
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
        let cards = Vec::from([Card{
            term: "hello".to_string(), definition: "bonjour".to_string()
        },
        Card{
            term: "goodbye".to_string(), definition: "au revoir".to_string()
        }]);

        assert_eq!(cards, parse_cards(text));
    }
}
