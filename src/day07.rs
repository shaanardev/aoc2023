use std::cmp::Ordering;

use itertools::Itertools;
use Card::*;

use crate::Solution;

#[derive(Eq, PartialEq, Debug, Ord, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Eq, PartialEq, Debug, Ord, PartialOrd, Hash)]
enum Card {
    Joker, 
    Num(u32),
    Jack,
    Queen,
    King,
    Ace,
}
impl Card {
    fn parse_cards(cards: &str) -> Vec<Card> {
        cards
            .chars()
            .filter_map(|c| c.try_into().ok())
            .collect()
    }

    fn parse_cards_p2(cards: &str) -> Vec<Card> {
        Self::parse_cards(cards)
            .into_iter()
            .map(|c| if c == Jack { Joker } else { c })
            .collect()
    }
}
impl TryFrom<char> for Card {
    type Error =();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Ace),
            'K' => Ok(King),
            'Q' => Ok(Queen),
            'J' => Ok(Jack),
            'T' => Ok(Num(10)),
            c => c.to_digit(10).map(|d| Num(d)).ok_or(()),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Hand {
    bid: i32,
    cards: Vec<Card>,
    h_type: HandType
}
impl Hand {
    fn new(bid: i32, cards: Vec<Card>, h_type: HandType) -> Hand {
        Hand {
            bid,
            cards,
            h_type,
        }
    }

    fn parse(input: &str) -> Self {
        let (card_input, bid_input) = input.split_once(" ").unwrap();
        let cards: Vec<Card> = Card::parse_cards(card_input);
        let hand_type = Hand::calculate_hand_type(&cards);
    
        Hand::new(bid_input.parse().unwrap(), cards, hand_type)
    } 

    fn parse_p2(input: &str) -> Self {
        let (card_input, bid_input) = input.split_once(" ").unwrap();
        let cards: Vec<Card> = Card::parse_cards_p2(card_input);
        let hand_type = Hand::calculate_hand_type(&cards);
    
        Hand::new(bid_input.parse().unwrap(), cards, hand_type)
    }

    fn calculate_hand_type(cards: &Vec<Card>) -> HandType {
        use HandType::*;
        let groups = cards.iter().counts();
        let distinct_count = groups.len();
        let max_group = groups.values().max().unwrap();
        let joker_count = groups.get(&Joker).unwrap_or(&0);

        match (distinct_count, max_group, joker_count) {
            (1, 5, _) => FiveOfAKind,  //
            (2, 4, 0) => FourOfAKind,  //
            (2, 4, _) => FiveOfAKind,  // Only 2 values, joker(s) change to match the 
                                       // other card(s)
            (2, 3, 0) => FullHouse,    //
            (2, 3, _) => FiveOfAKind,  // Only 2 values, jokers change to match the 
                                       // other cards
            (3, 3, 0) => ThreeOfAKind, //
            (3, 3, _) => FourOfAKind,  // Three jokers match a singleton, or single 
                                       // joker matches the triple
            (3, 2, 0) => TwoPair,      //
            (3, 2, 1) => FullHouse,    // Singleton joker matches one of the pairs
            (3, 2, 2) => FourOfAKind,  // Two jokers match the other pair
            (4, 2, 0) => OnePair,      //
            (4, 2, _) => ThreeOfAKind, // Two jokers match one of the singletons, 
                                       // a single joker matches the pair
            (5, 1, 0) => HighCard,     //
            (5, 1, 1) => OnePair,      // Joker pairs up with any one of the other values
            _ => unreachable!(),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.h_type
            .cmp(&other.h_type)
            .then(self.cards.cmp(&other.cards))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Day07 {
    p1: Vec<Hand>,
    p2: Vec<Hand>,
}
impl Solution for Day07 {
    type ParsedInput = Self;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        let p1 = input_lines
                                .lines()
                                .map(Hand::parse)
                                .collect::<Vec<_>>();
        let p2 = input_lines
                                .lines()
                                .map(Hand::parse_p2)
                                .collect::<Vec<_>>();
        Day07 { p1, p2 }
    }

    fn part_one(parsed_input: &mut Self::ParsedInput) -> String {
        let total = parsed_input
                            .p1
                            .iter()
                            .sorted()
                            .enumerate()
                            .map(|(i, hand)| (i + 1) as i32 * hand.bid)
                            .sum::<i32>();
        total.to_string()
    }

    fn part_two(parsed_input: &mut Self::ParsedInput) -> String {
        let total = parsed_input
                            .p2
                            .iter()
                            .sorted()
                            .enumerate()
                            .map(|(i, hand)| (i + 1) as i32 * hand.bid)
                            .sum::<i32>();
        total.to_string()
    }
}