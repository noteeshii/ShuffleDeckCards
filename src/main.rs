use rand::Rng;
use std::fmt;

const CARDS_IN_SUITE: usize = 13;

const CARDS_IN_DECK: usize = CARDS_IN_SUITE * 4;

struct Deck(Vec<Card>);

impl Deck {
    pub fn new() -> Deck {
        let mut cards = Vec::<Card>::with_capacity(CARDS_IN_DECK);

        cards.append(&mut Deck::build_suite(Suit::Heart));
        cards.append(&mut Deck::build_suite(Suit::Spade));
        cards.append(&mut Deck::build_suite(Suit::Diamond));
        cards.append(&mut Deck::build_suite(Suit::Club));

        Deck(cards)
    }

    fn build_suite(suit: Suit) -> Vec<Card> {
        let mut cards = Vec::<Card>::with_capacity(CARDS_IN_SUITE);

        for n in 1..CARDS_IN_SUITE {
            cards.push(Card::new(suit, n as i8));
        }

        cards
    }

    fn simple_shuffle(&mut self, times: usize) {
        let mut rng = rand::thread_rng();

        for _ in 0..times {
            let first_idx = rng.gen_range(0..self.0.len());
            let second_idx = rng.gen_range(0..self.0.len());

            self.0.swap(first_idx, second_idx);
        }
    }
}

impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for card in self.0.iter() {
            write!(f, "{} ", card)?;
        }

        Ok(())
    }
}

struct Card {
    suit: Suit,
    value: i8,
}

impl Card {
    fn new(suit: Suit, value: i8) -> Card {
        Card { suit, value }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self.value {
            1 => String::from("A"),
            11 => String::from("J"),
            12 => String::from("Q"),
            13 => String::from("K"),
            other_value => other_value.to_string(),
        };

        write!(f, "{}:{}", self.suit, value)
    }
}

#[derive(Clone, Copy)]
enum Suit {
    Heart,
    Spade,
    Diamond,
    Club,
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let icon = match self {
            Suit::Heart => "♥️",
            Suit::Spade => "♠️",
            Suit::Diamond => "♣️",
            Suit::Club => "♦️",
        };

        write!(f, "{}", icon)
    }
}

fn main() {
    let mut deck = Deck::new();

    println!("Initial: \n{}\n", deck);

    deck.simple_shuffle(10);

    println!("After 10 suffles: \n{}\n", deck);
}
