use std::ops::Range;

use crate::Solution;

#[derive(Default, Debug)]
struct Map {
    range: Range<i64>,
    delta: i64
}
impl Map {
    pub fn new(dest: i64, src: i64, range: i64) -> Map{
        Map {
            range: Range {
                start: src,
                end: src+range
            },
            delta: dest - src
        }
    }
}

#[derive(Default, Debug)]
struct Mappings {
    maps: Vec<Map>
}
impl Mappings {
    pub fn add(&mut self, dest: i64, src: i64, len: i64) {
        self.maps.push(Map::new(dest, src, len));
    }

    pub fn apply_map(&self, val: i64) -> i64 {
        for m in &self.maps {
            if m.range.contains(&val) {
                return val + m.delta;
            }
        }

        val
    }
}

#[derive(Debug, Default)]
pub struct Output {
    seeds: Vec<i64>,
    mappings: Vec<Mappings>
}

#[derive(Default, Debug)]
pub struct Day05;
impl Solution for Day05 {
    type ParsedInput = Output;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        let mut output = Output::default();
        let lines: Vec<String> = input_lines
                                .lines()
                                .map(String::from)
                                .filter(|s| !s.is_empty())
                                .collect();
        let seeds = lines[0].split_once(": ").unwrap().1;
        output.seeds = seeds.split(' ').map(|seed| seed.parse().unwrap()).collect();
        let mut curr_map = Mappings::default();
        for line in lines[2..].iter() {
            if line.contains(":") {
                output.mappings.push(curr_map);
                curr_map = Mappings::default();
                continue;
            }

            let nums: Vec<i64> = line.split(' ').map(|n| n.parse().unwrap()).collect();
            curr_map.add(nums[0], nums[1], nums[2]);
        }
        if !curr_map.maps.is_empty() {
            output.mappings.push(curr_map);
        }
        output
    }

    fn part_one(parsed_input: &mut Self::ParsedInput) -> String {
        let mut min = i64::MAX;

        for seed in parsed_input.seeds.iter() {
            let mut curr = *seed;
            for map in parsed_input.mappings.iter() {
                curr = map.apply_map(curr);
            }
            min = min.min(curr);
        }

        min.to_string()
    }

    fn part_two(parsed_input: &mut Self::ParsedInput) -> String {
        "".to_string()
    }
}