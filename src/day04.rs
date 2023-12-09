use crate::Solution;

#[derive(Clone, Debug)]
pub struct Card {
    id: usize,
    winning_numbers: Vec<usize>,
    matching_numbers: Vec<usize>
}

impl Card {
    fn new(id: usize, winning_numbers: Vec<usize>, matching_numbers: Vec<usize>) -> Self {
        Card {
            id,
            winning_numbers,
            matching_numbers,
        }
    }

    fn match_numbers(&self) -> usize {
        self.winning_numbers
            .iter()
            .filter(|&x| self.matching_numbers.contains(x))
            .cloned()
            .collect::<Vec<usize>>()
            .len()
    }
}

#[derive(Clone, Debug)]
pub struct Day04;

impl Solution for Day04 {
    type ParsedInput = Vec<Card>;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        let input_lines = input_lines.to_string();
        input_lines.lines()
            .map(|line| parse_line(&line).unwrap())
            .collect()
    }

    fn part_one(cards: &mut Self::ParsedInput) -> String {
        let sum: i64 = cards.iter()
            .map(|card| card.match_numbers())
            .filter(|&num_matches| num_matches > 0)
            .map(|num_matches| 2_i64.pow((num_matches as u32 - 1).into()))
            .sum();
        sum.to_string()
    }

    fn part_two(cards: &mut Self::ParsedInput) -> String {
        let mut copies = vec![1; cards.len()];
        for (i, card) in cards.iter().enumerate() {
            let total_matches = card.match_numbers();
            for j in 1..=total_matches {
                copies[i + j] += copies[i];
            }
        }
        let sum: usize = copies.iter().sum();
        sum.to_string()
    }
}

fn parse_line(line: &str) ->  Option<Card> {
    let parts: Vec<&str> = line.split(":").collect();
    let parts_len = parts.len();
    match parts_len {
        2 => {
            let id = parts[0]
                .trim_start_matches("Card ")
                .trim()
                .parse()
                .expect("Failed to parse id to a number");
    
            let numbers: Vec<usize> = parts[1]
                .split('|')
                .flat_map(|part| part.split_whitespace().filter_map(|s| s.parse().ok()))
                .collect();
    
            let split_index = numbers.len().min(10);
            let (winning_numbers, matching_numbers) = numbers.split_at(split_index);
    
            Some(Card::new(id, winning_numbers.to_vec(), matching_numbers.to_vec()))
        },
        _ => {
            panic!("Error with parsing file: expected 2 parts, got {}", parts_len);
        },
    }    
}