use std::collections::VecDeque;

use crate::Solution;

pub struct History(Vec<isize>);

#[derive(Debug)]
pub struct Day09;
impl Solution for Day09 {
    type ParsedInput = Vec<History>;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        let lines = input_lines.lines().map(String::from).collect::<Vec<_>>();
        lines.iter().map(|line| {
            let numbers: Vec<isize> = line.split_whitespace()
                .map(|s| s.parse::<isize>().unwrap())
                .collect();
            History(numbers)
        }).collect::<Vec<History>>()
    }

    fn part_one(parsed_input: &mut Self::ParsedInput) -> String {
        let total: isize = parsed_input 
                            .iter()
                            .map(|History(vec)| find_next(vec, &mut VecDeque::from(vec![vec[vec.len() - 1]])))
                            .sum::<isize>();
        total.to_string()
    }

    fn part_two(parsed_input: &mut Self::ParsedInput) -> String {
        let total: isize = parsed_input 
                            .iter()
                            .map(|History(vec)| find_prev(vec, &mut VecDeque::from(vec![vec[0]])))
                            .sum::<isize>();
        total.to_string()
    }
}

fn find_next(history: &[isize], lasts: &mut VecDeque<isize>) -> isize {
    let diffs = history.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();
    if diffs.iter().sum::<isize>() == 0 {
        return lasts.iter().sum::<isize>();
    }
    lasts.push_back(diffs[diffs.len() - 1]);
    find_next(&diffs, lasts)
}

fn find_prev(history: &[isize], firsts: &mut VecDeque<isize>) -> isize {
    let diffs = history.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();
    if diffs.iter().sum::<isize>() == 0 {
        return firsts.iter().rev().fold(0, |acc, x| x - acc);
    }
    firsts.push_back(diffs[0]);
    find_prev(&diffs, firsts)
}