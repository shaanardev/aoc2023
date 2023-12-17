pub mod day00;
pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;

pub trait Solution {
    type ParsedInput;

    fn parse_input(input_lines: &str) -> Self::ParsedInput;
    
    fn part_one(parsed_input: &mut Self::ParsedInput) -> String;
    fn part_two(parsed_input: &mut Self::ParsedInput) -> String;

    fn solve_part_one(input_lines: &str) -> String {
        Self::part_one(&mut Self::parse_input(input_lines))
    }

    fn solve_part_two(input_lines: &str) -> String {
        Self::part_two(&mut Self::parse_input(input_lines))
    }

    fn solve(input_lines: &str) -> (String, String) {
        let mut input = Self::parse_input(input_lines);
        let p1 = Self::part_one(&mut input);
        let p2 = Self::part_two(&mut input);
        println!("----------");
        println!("Part 1: {}\nPart 2: {}", p1, p2);
        (p1, p2)
    }
}

pub fn solve_day(day: &i32) {
    match day {
        0 => day00::Day00::solve(include_str!("../inputs/0")),
        1 => day01::Day01::solve(include_str!("../inputs/1")),
        2 => day02::Day02::solve(include_str!("../inputs/2")),
        3 => day03::Day03::solve(include_str!("../inputs/3")),
        4 => day04::Day04::solve(include_str!("../inputs/4")),
        5 => day05::Day05::solve(include_str!("../inputs/5")),
        6 => day06::Day06::solve(include_str!("../inputs/6")),
        7 => day07::Day07::solve(include_str!("../inputs/7")),
        8 => day08::Day08::solve(include_str!("../inputs/8")),
        9 => day09::Day09::solve(include_str!("../inputs/9")),
        _ => panic!("Day not found"),
    };
}