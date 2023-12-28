use std::{collections::HashSet, hash::Hasher};
use crate::Solution;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Board {
    grid: Vec<Vec<char>>
}

impl Board {
    fn roll_all_without_total(&mut self) {
        self.roll_north();
        self.roll_west();
        self.roll_south();
        self.roll_east();
    }

    // Walks through the grid looking for 'O'
    // its total is number of rows minus the row it's on.
    fn calculate_total(&self) -> usize {
        self.grid
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter().enumerate().map(move |(_, c)| if *c == 'O' { self.grid.len() - y } else { 0 })
            })
            .sum()
    }

    fn roll_north(&mut self) -> usize{
        let mut total = 0;
        for x in 0..self.grid[0].len() {
            let mut next = 0;
            for y in 0..self.grid.len() {
                match self.grid[y][x] {
                    'O' => {
                        (self.grid[next][x], self.grid[y][x]) = 
                            (self.grid[y][x], self.grid[next][x]);
                        
                        total += self.grid.len() - next;
                        next += 1;

                        while next < y && self.grid[next][x] != '.' {
                            next += 1;
                        }
                    }
                    '#' => next = y + 1,
                    _ => {}
                }
            }
        }
        total
    }

    fn roll_south(&mut self) -> usize {
        let mut total = 0;
        for x in 0..self.grid[0].len() {
            let mut next = self.grid.len() - 1;
            for y in (0..self.grid.len()).rev() {
                match self.grid[y][x] {
                    'O' => {
                        (self.grid[next][x], self.grid[y][x]) =
                            (self.grid[y][x], self.grid[next][x]);
                        
                        total += self.grid.len() - next;  
                        next = next.saturating_sub(1);

                        while next > y && self.grid[next][x] != '.' {
                            next = next.saturating_sub(1);
                        }
                    }
                    '#' => next = y.saturating_sub(1),
                    _ => {}
                }
            }
        }
        total
    }

    fn roll_west(&mut self) -> usize {
        let mut total = 0;
        for y in 0..self.grid.len() {
            let mut next = 0;
            for x in 0..self.grid[0].len() {
                match self.grid[y][x] {
                    'O' => {
                        (self.grid[y][next], self.grid[y][x]) =
                            (self.grid[y][x], self.grid[y][next]);

                        total += self.grid.len() - next;  
                        next += 1;

                        while next < x && self.grid[y][next] != '.' {
                            next += 1;
                        }
                    }
                    '#' => next = x + 1,
                    _ => {}
                }
            }
        }

        total
    }
    fn roll_east(&mut self) -> usize {
        let mut total = 0;
        for y in 0..self.grid.len() {
            let mut next = self.grid[0].len() - 1;
            for x in (0..self.grid[0].len()).rev() {
                match self.grid[y][x] {
                    'O' => {
                        (self.grid[y][next], self.grid[y][x]) =
                            (self.grid[y][x], self.grid[y][next]);
                        
                        total += self.grid.len() - next;  
                        next = next.saturating_sub(1);

                        while next > x && self.grid[y][next] != '.' {
                            next = next.saturating_sub(1);
                        }
                    }
                    '#' => next = x.saturating_sub(1),
                    _ => {}
                }
            }
        }

        total
    }
}

// Used for part 2 so we can track which cycles we have seen
// to reduce the number of iterations down from 1billion
#[derive(Debug, Eq)]
pub struct BoardState {
    step: usize,
    board: Board,
}
impl BoardState {
    fn new(step: usize, board: Board) -> Self {
        Self { step, board }
    }
}

impl PartialEq for BoardState {
    fn eq(&self, other: &Self) -> bool {
        self.board == other.board
    }
}

impl std::hash::Hash for BoardState {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.board.hash(state);
    }
}

#[derive(Debug)]
pub struct Day14;
impl Solution for Day14 {
    type ParsedInput = Board;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        let grid = input_lines
                    .lines()
                    .map(|line| line.chars().collect::<Vec<_>>())
                    .collect::<Vec<_>>();
        Board { grid }
    }

    fn part_one(board: &mut Self::ParsedInput) -> String {
        board.roll_north().to_string()
    }

    fn part_two(board: &mut Self::ParsedInput) -> String {
        p2_helper(board).to_string()
    }
}

fn p2_helper(board: &mut Board) -> usize {
    let mut seen_cycles = HashSet::new();

    for i in 0..1_000_000_000 {
        seen_cycles.insert(BoardState::new(i, board.clone()));

        board.roll_all_without_total();

        if let Some(state) = seen_cycles.get(&BoardState::new(0, board.to_owned())) {
            let cycle_len = i + 1 - state.step;
            let remaining = 1_000_000_000 - i - 1;
            let remaining = remaining % cycle_len;

            // Remaining is the number of steps we need to take to
            // from where we are at to get to the same position that
            // 1_000_000_000 steps would have taken us.
            for _ in 0..remaining {
                board.roll_all_without_total();
            }

            return board.calculate_total();
        }
    }

    panic!("took too long?");
}