use rand::prelude::*;

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
    rank: Rank,
    suit: Suit,
}

impl Card {
    fn new(suit: Suit, rank: Rank) -> Self {
        Self { suit, rank }
    }
}

type DeckType = VecDeque<Card>;

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

    pub fn from(cards: DeckType) -> Self {
        Self {
            cards: cards.clone(),
            count: cards.len() as u8,
        }
    }

    pub fn shuffle(&mut self) {
        for _ in 0..=50 {
            let r1 = rand::thread_rng().gen_range(0, 52);
            let r2 = rand::thread_rng().gen_range(0, 52);
            self.cards.swap(r1, r2);
        }
    }

    pub fn deal_one(&mut self) -> Option<Card> {
        if self.count > 0 {
            self.count -= 1;
            self.cards.pop_front()
        } else {
            None
        }
    }

    pub fn deal_few(&mut self, amount: u8) -> Option<DeckType> {
        let mut dealt_cards: DeckType = VecDeque::new();
        if self.count > amount {
            for _ in 1..=amount {
                dealt_cards.push_front(self.deal_one().unwrap());
            }
            Some(dealt_cards)
        } else {
            None
        }
    }
}

fn main() {
    let mut deck = Deck::new();
    deck.shuffle();

    let ten = deck.deal_few(10).unwrap();
    println!("{:?}", &ten);

    let d2 = Deck::from(ten);
    println!("{:?}", d2.cards)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deck_init() {
        let deck = Deck::new();
        assert_eq!(deck.count, 52);
    }

    #[test]
    fn deck_deal_one() {
        let mut deck = Deck::new();
        let _d = deck.deal_one();
        assert_eq!(deck.count, 51);
    }

    #[test]
    fn deck_deal_few() {
        let mut deck = Deck::new();
        let d = deck.deal_few(10);
        assert_eq!(deck.count, 42);
        assert_eq!(d.unwrap().len(), 10);
    }

    #[test]
    fn deck_deal_few_over() {
        let mut deck = Deck::new();
        let d = deck.deal_few(100);
        assert!(d.is_none());
    }
}
