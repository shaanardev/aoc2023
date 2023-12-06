use crate::Solution;
use regex::Regex;

#[derive(Clone, Debug)]
pub struct Day02;

#[derive(Clone, Debug, Copy)]
pub struct Cubes {
    red: i32,
    blue: i32,
    green: i32,
}
impl Cubes {
    fn new(line: &str) -> Cubes {
        let mut cubes = Cubes {
            red: 0,
            blue: 0,
            green: 0,
        };
        let re = Regex::new(r"(?<count>[0-9]+) (?<colour>[a-zA-Z]+)").unwrap();

        for cap in re.captures_iter(line) {
            let colour = cap.name("colour").unwrap().as_str();
            let count = cap.name("count").unwrap().as_str().parse::<i32>().unwrap();
            match colour {
                "red" => cubes.red = count,
                "green" => cubes.green = count,
                "blue" => cubes.blue = count,
                _ => panic!("Unexpected colour {}", colour),
            }
        }

        cubes
    }

    pub fn are_cubes_valid_for(&self, cubes_to_verify: &Cubes) -> bool {
        self.red <= cubes_to_verify.red &&
        self.blue <= cubes_to_verify.blue &&
        self.green <= cubes_to_verify.green
    }

    pub fn combine_with_least_cubes(&self, least_cubes: &mut Cubes) {
        if self.red > least_cubes.red {
            least_cubes.red = self.red
        }
        if self.green > least_cubes.green {
            least_cubes.green = self.green
        }
        if self.blue > least_cubes.blue {
            least_cubes.blue = self.blue
        }
    } 

    pub fn power(&self) -> i32 {
        self.red * self.blue * self.green
    }

}

#[derive(Clone, Debug)]
pub struct Game {
    id: i32,
    rounds: Vec<Cubes>
}
impl Game {
    fn new(line: &str) -> Game {
        let re = Regex::new(r"Game (?<id>[0-9]+): (?<rounds>.+)").unwrap();
        let Some(caps) = re.captures(line) else {
            panic!("There was no game")
        };

        let id = caps["id"].parse::<i32>().unwrap();
        let input_rounds = &caps["rounds"];
        let rounds = input_rounds
                    .split("; ")
                    .map(Cubes::new)
                    .collect::<Vec<Cubes>>();

        Game { id, rounds }
    }

    pub fn are_cubes_valid_for(&self, cubes_to_verify: &Cubes) -> bool {
        self.rounds.iter().all(|&c| c.are_cubes_valid_for(cubes_to_verify))
    }

    pub fn get_least_cubes_combination(&self) -> Cubes {
        let mut least_cubes = Cubes::new("");
        self.rounds.iter().for_each(|c| c.combine_with_least_cubes(&mut least_cubes));
        least_cubes
    }

    pub fn least_cubes_power(&self) -> i32 {
        self.get_least_cubes_combination().power()
    }
}

impl Solution for Day02 {
    type ParsedInput = Vec<Game>;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        let input_lines = input_lines.to_string();
        input_lines.lines().map(Game::new).collect::<Vec<Game>>()
    }

    
    fn part_one(parsed_input: &mut Self::ParsedInput) -> String {
        let verify_cubes = Cubes {
            red: 12,
            green: 13,
            blue: 14,
        };

        parsed_input.iter()
            .filter_map(|game| if game.are_cubes_valid_for(&verify_cubes) { Some(game.id) } else { None })
            .sum::<i32>()
            .to_string()
    }

    fn part_two(parsed_input: &mut Self::ParsedInput) -> String {
        let mut sum = 0;
        for game in parsed_input {
            sum += game.least_cubes_power();
        }
        sum.to_string()
    }
}