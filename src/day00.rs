use crate::Solution;

#[derive(Clone, Debug)]
pub struct Day00;

impl Solution for Day00 {
    type ParsedInput = String;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        input_lines.to_string()
    }

    fn part_one(parsed_input: &mut Self::ParsedInput) -> String {
        format!(
            "{}",
            parsed_input.lines().map(sum_numbers_in_line).sum::<i32>()
        )
    }

    fn part_two(parsed_input: &mut Self::ParsedInput) -> String {
        format!(
            "{}",
            parsed_input
                .lines()
                .map(square_difference_in_line)
                .sum::<i32>()
        )
    }
}

fn sum_numbers_in_line(line: &str) -> i32 {
    line.split(", ")
        .map(|number| number.parse::<i32>().expect("Couldn't parse"))
        .sum::<i32>()
}

fn square_difference_in_line(line: &str) -> i32 {
    let numbers = line
        .split(", ")
        .map(|number| number.parse::<i32>().expect("Couldn't parse"))
        .collect::<Vec<i32>>();
    assert_eq!(numbers.len(), 2);
    (numbers.first().unwrap() - numbers.get(1).unwrap()).pow(2)
}