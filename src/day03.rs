use std::collections::HashSet;

use crate::Solution;

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub struct Coord {
    x: isize,
    y: isize
}
impl Coord {
    pub fn get_neighbours(&self) -> Vec<Coord> {
        let directions = vec![-1, 0, 1];
        directions
            .iter()
            .flat_map(|&dx| directions.iter().map(move |&dy| (dx, dy)))
            .filter(|&(dx, dy)| dx != 0 || dy != 0)
            .map(|(dx, dy)| Coord { x: self.x + dx, y: self.y + dy })
            .collect()
    }

    pub fn neighbours_hash_set(&self) -> HashSet<Coord> {
        HashSet::from_iter(self.get_neighbours())
    }
}

#[derive(Clone, Debug)]
pub struct Matrix(Vec<Vec<char>>);
impl Matrix {
    fn get(&self) -> &Vec<Vec<char>> {
        &self.0
    }

    fn max_y(&self) -> usize {
        self.get().len()
    }

    fn max_x(&self) -> usize {
        self.get()[0].len()
    }

    fn is_start_of_number(&self, x: usize, y: usize) -> bool {
        let matrix = self.get();
        matrix[y][x].is_ascii_digit() && ((x == 0) || !matrix[y][x - 1].is_ascii_digit())
    }

    fn is_symbol(&self, coord: &Coord) -> bool {
        let val = self.get()[coord.y as usize][coord.x as usize];
        !val.is_ascii_digit() && val != '.'
    }

    // PART 1

    fn get_numbers(&self) -> Vec<Number> {
        self.get()
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter(move |number| self.is_start_of_number(number.0, y))
                    .filter_map(move |(x, _)| self.get_number(x, y))
            })
            .collect::<Vec<Number>>()
    }

    fn get_number(&self, x: usize, y: usize) -> Option<Number> {
        let matrix = self.get();
        let current = matrix[y][x];
        if !current.is_ascii_digit() {
            return None;
        }

        let mut number_value = String::from(current);
        let mut coords = HashSet::new();
        coords.insert(Coord {
            x: x as isize,
            y: y as isize,
        });

        let mut next_x = x + 1;
        while let Some(cell) = matrix[y].get(next_x) {
            if !cell.is_ascii_digit() {
                break;
            }

            coords.insert(Coord {
                x: next_x as isize,
                y: y as isize,
            });

            number_value.push(*cell);
            next_x += 1;
        }

        Some(Number {
            value: number_value.parse().unwrap_or_default(),
            coords,
        })
    }

    fn get_neighboring_numbers(&self, number: &Number) -> HashSet<Coord> {
        let mut neighbors = HashSet::new();

        for coord in number.coords.iter() {
            let coord_neighbors = self
                .get_neighboring_points(coord)
                .into_iter()
                .filter(|point| (!number.coords.contains(point)))
                .filter(|point| (!neighbors.contains(point)))
                .collect::<HashSet<Coord>>();

            neighbors.extend(coord_neighbors);
        }
        neighbors
    }

    pub fn get_part_numbers(&self) -> Vec<Number> {
        let mut part_numbers = Vec::new();

        for number in self.get_numbers() {
            let neighbors = self.get_neighboring_numbers(&number);
            if neighbors.iter().any(|neighbor| self.is_symbol(neighbor)) {
                part_numbers.push(number);
            }
        }

        part_numbers
    }

    fn get_neighboring_points(&self, coord: &Coord) -> HashSet<Coord> {
        coord
            .neighbours_hash_set()
            .into_iter()
            .filter(|coord| self.does_coord_exist(coord))
            .collect::<HashSet<Coord>>()
    }

    fn does_coord_exist(&self, coord: &Coord) -> bool {
        (coord.x >= 0)
            && (coord.x < self.max_x() as isize)
            && (coord.y >= 0)
            && (coord.y < self.max_y() as isize)
    }

    // PART 2
    fn get_surrounding_numbers<'a>(&self, coord: &Coord, numbers: &'a[Number]) -> Vec<&'a Number>{
        let neighbors: HashSet<Coord> = self.get_neighboring_points(coord);
        numbers.iter()
                .filter(|&part_number| !part_number.coords.is_disjoint(&neighbors))
                .collect()
    }

    fn get_possible_gears(&self) -> Vec<Coord> {
        (0..self.max_y())
            .flat_map(|y| 
                (0..self.max_x())
                    .filter(move |&x| self.0[y][x] == '*')
                    .map(move |x| Coord { x: x as isize, y: y as isize }))
            .collect()
    }

    fn get_gear_ratios(&self) -> Vec<isize> {
        let mut gear_ratios = vec![];

        let possible_gears = self.get_possible_gears();

        let part_numbers = self.get_part_numbers();

        for potential_gear in possible_gears {
            let matching_part_numbers =
                self.get_surrounding_numbers(&potential_gear, &part_numbers);

            if matching_part_numbers.len() == 2 {
                gear_ratios.push(matching_part_numbers[0].value * matching_part_numbers[1].value);
            }
        }

        gear_ratios
    }

}

#[derive(Clone, Debug)]
pub struct Number {
    value: isize,
    coords: HashSet<Coord>,
}

#[derive(Copy, Clone, Debug)]
pub struct Day03;

impl Solution for Day03 {
    type ParsedInput = Matrix;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        let input_lines = input_lines.to_string();
        let data = input_lines
            .lines()
            .map(String::from)
            .map(|x| x.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        Matrix(data)
    }

    fn part_one(matrix: &mut Self::ParsedInput) -> String {
        let part_numbers = matrix.get_part_numbers();

        let sum: isize = part_numbers
            .iter()
            .map(|part_number| part_number.value)
            .sum();
        sum.to_string()
    }

    fn part_two(matrix: &mut Self::ParsedInput) -> String {
        let sum: isize = matrix.get_gear_ratios().iter().sum();
        sum.to_string()
    }
}