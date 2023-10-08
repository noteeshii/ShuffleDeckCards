const CARDS_IN_SUITE: usize = 13;

const CARDS_IN_DECK: usize = CARDS_IN_SUITE * 4;

#[derive(Debug)]
struct Deck(Vec<Card>);

impl Deck {
    pub fn new() -> Deck {
        let mut cards = Vec::<Card>::with_capacity(CARDS_IN_DECK);

        Deck(cards)
    }

    fn build_suite() -> Vec<Card> {
        let mut cards = Vec::<Card>::with_capacity(CARDS_IN_SUITE);

        cards
    }
}

#[derive(Debug)]
struct Card {
    suit: Suit,
    value: i8,
}

#[derive(Debug)]
enum Suit {
    Heart,
    Spade,
    Diamond,
    Club,
}

fn main() {
    let mut deck = Deck::new();

    println!("{:?}", deck);
}
