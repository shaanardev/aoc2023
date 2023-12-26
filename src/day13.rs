use crate::Solution;

#[derive(Debug)]
pub struct Note {
    grid: Vec<Vec<char>>
}
impl From<&str> for Note {
    fn from(s: &str) -> Self {
        let grid = s.lines().map(|line| line.chars().collect()).collect();
        Self { grid }
    }
}
impl Note {
    // problem statement implies there is only one reflection per grid
    // "perfect reflection" implies reflection spans across as much of the grid as possible
    fn find_reflections_p1(&self) -> usize {
        'rows: for i in 0..self.grid.len() - 1 {
            if self.is_row_equal(i, i + 1) {
                let dist_to_edge = i.min(self.grid.len() - i - 2);
                for d in 1..=dist_to_edge {
                    if !self.is_row_equal(i - d, i + d + 1) {
                        continue 'rows;
                    }
                }

                return (i + 1) * 100;
            }
        }
        

        'columns: for i in 0..self.grid[0].len() - 1 {
            if self.is_col_equal(i, i + 1) {
                let dist_to_edge = i.min(self.grid[0].len() - i - 2);
                for d in 1..=dist_to_edge {
                    if !self.is_col_equal(i - d, i + d + 1) {
                        continue 'columns;
                    }
                }
                return i + 1;
            }
        }

        0
    }
    
    //now we want to know if the difference in the two sides of the reflection are 1.
    fn find_reflections_p2(&self) -> usize {
        'rows: for i in 0..self.grid.len() - 1 {
            let mut diff = self.diff_rows(i, i + 1);
            if diff <= 1 {
                let min_distance_to_edge = i.min(self.grid.len() - i - 2);
                for d in 1..=min_distance_to_edge {
                    diff += self.diff_rows(i - d, i + d + 1);
                    if diff > 1 {
                        continue 'rows;
                    }
                }

                if diff == 0 {
                    continue 'rows;
                }
                return (i + 1) * 100;
            }
        }

        'columns: for i in 0..self.grid[0].len() - 1 {
            let mut diff = self.diff_columns(i, i + 1);
            if diff <= 1 {
                let min_distance_to_edge = i.min(self.grid[0].len() - i - 2);
                for d in 1..=min_distance_to_edge {
                    diff += self.diff_columns(i - d, i + d + 1);
                    if diff > 1 {
                        continue 'columns;
                    }
                }
                if diff == 0 {
                    continue 'columns;
                }
                return i + 1;
            }
        }

        0
    }

    //p1 helpers
    fn is_row_equal(&self, y1: usize, y2: usize) -> bool {
        self.grid[y1] == self.grid[y2]
    }

    fn is_col_equal(&self, x1: usize, x2: usize) -> bool {
        self.grid.iter().all(|line| line[x1] == line[x2])
    }

    //p2 helpers
    fn diff_rows(&self,  y1: usize, y2: usize) -> usize {
        (0..self.grid[0].len())
            .filter_map(|x| if self.grid[y1][x] != self.grid[y2][x] { Some(1) } else { None })
            .sum::<usize>()
    }

    fn diff_columns(&self, x1: usize, x2: usize) -> usize {
        (0..self.grid.len())
            .filter_map(|y| if self.grid[y][x1] != self.grid[y][x2] { Some(1) } else { None })
            .sum::<usize>()
    }
}


#[derive(Debug)]
pub struct Day13;
impl Solution for Day13 {
    type ParsedInput = Vec<Note>;

    fn parse_input(input: &str) -> Self::ParsedInput {
        input
        .split("\r\n\r\n") //should be \n\n something funky going on with the way I copied the input to a file
        .map(|note| note.into())
        .collect::<Vec<Note>>()
    }

    fn part_one(notes: &mut Self::ParsedInput) -> String {
        notes.iter().map(|note| note.find_reflections_p1()).sum::<usize>().to_string()
    }

    fn part_two(notes: &mut Self::ParsedInput) -> String {
        notes.iter().map(|note| note.find_reflections_p2()).sum::<usize>().to_string()
    }
}
