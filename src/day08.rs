use std::collections::HashMap;

use crate::Solution;

#[derive(Debug)]
pub struct Node {
    left: String,
    right: String
}

#[derive(Debug, Default)]
pub struct Map(HashMap<String, Node>);
impl Map {
    fn add(&mut self, input: &str) {
        let split_input = input.split('=').collect::<Vec<_>>();

        let name = split_input[0].trim().to_string();
        let map = split_input[1]
                                    .trim()
                                    .split(',')
                                    .map(|s| s.trim().trim_start_matches('(').trim_end_matches(')'))
                                    .collect::<Vec<_>>();
        assert_eq!(map.len(), 2);
        let node = Node {
            left: map[0].to_string(),
            right: map[1].to_string()
        };
        self.0.insert(name.trim().to_string(), node);
        return;
    }
}
#[derive(Debug)]
pub struct Input {
    instructions: Vec<char>,
    map: Map
}
impl Input {
    fn traverse_until_end<F>(&self, start: &String, end: F) -> usize 
    where  
        F: Fn(&String) -> bool,
    {
        let mut curr = start;
        let mut steps = 0;

        while !end(curr) {
            let index = steps % self.instructions.len();
            let instruction = self.instructions[index];

            let branch = self.map.0.get(curr).unwrap();
            curr = match instruction {
                'L' => &branch.left,
                'R' => &branch.right,
                _ => panic!("Error {:?}", self),
            };

            steps += 1;
        }

        steps
    }
}
#[derive(Debug)]
pub struct Day08;
impl Solution for Day08 {
    type ParsedInput = Input;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        let lines = input_lines.lines().filter(|s| !s.is_empty()).collect::<Vec<_>>();
        let instructions = lines[0].to_string().chars().collect::<Vec<_>>();
        let mut map = Map::default();
        for line in lines.iter().skip(1) {
            map.add(line);
        }

        Input { instructions, map }
    }

    fn part_one(parsed_input: &mut Self::ParsedInput) -> String {
        let steps = parsed_input.traverse_until_end(&"AAA".to_string(), |s| s == "ZZZ");
        steps.to_string()
    }

    fn part_two(parsed_input: &mut Self::ParsedInput) -> String {
        let starts = parsed_input.map.0
                                        .keys()
                                        .filter(|k| k.ends_with('A'))
                                        .collect::<Vec<_>>();
        let ends = parsed_input.map.0
                                        .keys()
                                        .filter(|k| k.ends_with('Z'))
                                        .collect::<Vec<_>>();
                                    
        let mut steps = vec![];
        for start in starts {
            let step = parsed_input.traverse_until_end(start, |s| ends.contains(&&s));
            steps.push(step);
        }
        let total = lcm(&steps);
        total.to_string()
    }
}

// https://www.youtube.com/watch?v=t5ktQvHJG2Y
// https://github.com/TheAlgorithms/Rust/blob/master/src/math/lcm_of_n_numbers.rs
pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}