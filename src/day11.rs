use std::ops::{Add, Sub};

use itertools::Itertools;

use crate::Solution;

#[derive(Debug, Copy, Clone)]
pub struct Coord {
    x: i64,
    y: i64
}
impl Coord {
    fn new(x: i64, y: i64) -> Coord {
        Coord { x, y }
    }

    fn abs(&self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }
}
impl Add for Coord {
    type Output = Self;
 
    fn add(self, other: Self) -> Self {
        Coord {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl Sub for Coord {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Coord {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}

#[derive(Debug)]
pub struct Input {
    empty_rows: Vec<usize>,
    empty_columns: Vec<usize>,
    galaxies: Vec<Coord>
}
impl Input {
    fn expand_galaxy_on_coord(&self, coord: &Coord) -> Coord {
        let expand_steps_row = self.empty_rows
            .iter()
            .position(|row| {
                row > &(coord.y as usize)
            })
            .unwrap_or(self.empty_rows.len());
        let expand_steps_columns = self.empty_columns
            .iter()
            .position(|column| {
                column > &(coord.x as usize)
            })
            .unwrap_or(self.empty_columns.len());

        *coord + Coord::new(expand_steps_columns as i64,  expand_steps_row as i64)
    }

    fn expand_galaxy_by_size_on_coord(&self, coord: &Coord, expansion_size: i64) -> Coord {
        let expand_steps_row = self.empty_rows
            .iter()
            .position(|row| {
                row > &(coord.y as usize)
            })
            .unwrap_or(self.empty_rows.len());
        let expand_steps_columns = self.empty_columns
            .iter()
            .position(|column| {
                column > &(coord.x as usize)
            })
            .unwrap_or(self.empty_columns.len());

        *coord + Coord::new(expand_steps_columns as i64 * (expansion_size-1),  expand_steps_row as i64 * (expansion_size-1))
    }
}

#[derive(Debug)]
pub struct Day11;
impl Solution for Day11 {
    type ParsedInput = Input;

    fn parse_input(input: &str) -> Self::ParsedInput {
        let empty_rows = input
                            .lines()
                            .enumerate()
                            .filter_map(|(idx, line)| {
                                line.chars().all(|c| c == '.').then_some(idx)
                            })
                            .collect::<Vec<_>>();
        let mut columns = input
                            .lines()
                            .map(|line| line.chars())
                            .collect::<Vec<_>>();
        let empty_columns = std::iter::from_fn(move || {
            let mut items = vec![];
            for iter in &mut columns {
                match iter.next() {
                    Some(item) => {
                        items.push(item)
                    },
                    None => return None,
                }
            }
            Some(items)
        })
        .enumerate()
        .filter_map(|(idx, column)| {
            column.iter().all(|c| c == &'.').then_some(idx)
        })
        .collect::<Vec<_>>();

        let galaxies = input    
                        .lines()
                        .enumerate()
                        .flat_map(|(y, line)| {
                            line.chars().enumerate().filter_map(move |(x, c)| {
                                match c {
                                    '#' => Some(Coord::new(x as i64, y as i64)),
                                    _ => None
                                }
                            })
                        })
                        .collect::<Vec<_>>();
        Input {
            empty_rows,
            empty_columns,
            galaxies
        }
    }

    fn part_one(input: &mut Self::ParsedInput) -> String {
        let count = input.galaxies
                        .iter()
                        .combinations(2)
                        .map(|s| {
                            let a = s[0];
                            let b = s[1];
                            let expanded_a = input.expand_galaxy_on_coord(a);
                            let expanded_b = input.expand_galaxy_on_coord(b);
                            let v = (expanded_a - expanded_b).abs();
                            let distance = (v.x + v.y).abs();
                            distance
                        })
                        .sum::<i64>();
        count.to_string()
    }

    fn part_two(input: &mut Self::ParsedInput) -> String {
        let expansion_size: i64 = 1_000_000;
        let count = input.galaxies
                        .iter()
                        .combinations(2)
                        .map(|s| {
                            let a = s[0];
                            let b = s[1];
                            let expanded_a = input.expand_galaxy_by_size_on_coord(a, expansion_size);
                            let expanded_b = input.expand_galaxy_by_size_on_coord(b, expansion_size);
                            let v = (expanded_a - expanded_b).abs();
                            let distance = (v.x + v.y).abs();
                            distance
                        })
                        .sum::<i64>();
        count.to_string()
    }
}