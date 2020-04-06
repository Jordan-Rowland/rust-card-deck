use rand::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;

use std::collections::VecDeque;

const SUIT_LIST: [&str; 4] = ["C", "D", "H", "S"];
const RANK_LIST: [&str; 13] = [
    "II", "III", "IV", "V", "VI", "VII", "VIII", "IX", "X", "J", "Q", "K", "A",
];

#[derive(Debug, Clone, Copy)]
enum Suit {
    C,
    D,
    H,
    S,
}

#[derive(Debug, Clone, Copy)]
enum Rank {
    II,
    III,
    IV,
    V,
    VI,
    VII,
    VIII,
    IX,
    X,
    J,
    Q,
    K,
    A,
}

#[derive(Debug, Clone, Copy)]
struct Card {
    suit: Suit,
    rank: Rank,
}

impl Card {
    fn new(suit: Suit, rank: Rank) -> Self {
        Self { suit, rank }
    }
}

type DeckType = VecDeque<Card>;

// impl SliceRandom for DeckType {}

#[derive(Debug)]
struct Deck {
    cards: DeckType,
    count: u8,
}

impl Deck {
    pub fn new() -> Self {
        let mut _cards: DeckType = VecDeque::new();
        let mut s: Suit = Suit::C;
        let mut r: Rank = Rank::II;
        for _s in SUIT_LIST.iter() {
            match _s {
                &"C" => s = Suit::C,
                &"D" => s = Suit::D,
                &"H" => s = Suit::H,
                &"S" => s = Suit::S,
                _ => (),
            };
            for _r in RANK_LIST.iter() {
                match _r {
                    &"II" => r = Rank::II,
                    &"III" => r = Rank::III,
                    &"IV" => r = Rank::IV,
                    &"V" => r = Rank::V,
                    &"VI" => r = Rank::VI,
                    &"VII" => r = Rank::VII,
                    &"VIII" => r = Rank::VIII,
                    &"IX" => r = Rank::IX,
                    &"X" => r = Rank::X,
                    &"J" => r = Rank::J,
                    &"Q" => r = Rank::Q,
                    &"K" => r = Rank::K,
                    &"A" => r = Rank::A,
                    _ => (),
                };
                _cards.push_back(Card::new(s, r))
            }
        }
        Self {
            cards: _cards.clone(),
            count: _cards.len() as u8,
        }
    }

    pub fn shuffle(&mut self) {
        rand::thread_rng().gen_range(1, 53);
        // let mut rng = thread_rng();
        &self.cards;
    }

    pub fn deal_one(&mut self) -> Option<Card> {
        if self.count > 0 {
            self.count -= 1;
            self.cards.pop_front()
        } else {
            None
        }
    }

    // pub fn deal_few(&mut self, amount: u8) -> Card {}
}

fn main() {
    let mut deck = Deck::new();
    // let card = deck.deal_one();
    // println!("{:?}", card);
    // let card = deck.deal_one();
    // println!("{:?}", card);
    // let card = deck.deal_one();
    // println!("{:?}", card);
    // let card = deck.deal_one();
    // println!("{:?}", card);
    // let card = deck.deal_one();
    // println!("{:?}", card);

    // println!("{:?}", deck);
    // println!("{:?}", card);
    // println!("{:?}", deck);
}
