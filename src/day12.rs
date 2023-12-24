use std::collections::HashMap;

use crate::Solution;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, one_of, space1},
    multi::{many1, separated_list1},
    IResult,
};

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
enum SpringType {
    Operational,
    Damaged,
    Unknown,
}
impl From<char> for SpringType {
    fn from(value: char) -> Self {
        use SpringType::*;
        match value {
            '.' => Operational,
            '#' => Damaged,
            _ => Unknown,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct Record {
    springs: Vec<SpringType>,
    groups: Vec<usize>
}
impl Record {
    fn new(springs: Vec<SpringType>, groups: Vec<usize>) -> Self {
        Self { springs, groups }
    }

    fn parse_from_line(input: &str) -> IResult<&str, Record> {
        let (input, springs) = many1(one_of(".#?"))(input)?;
        let (input, _) = space1(input)?;
        let (input, groups) = separated_list1(tag(","), digit1)(input)?;

        Ok((
            input,
            Record {
                springs: springs.into_iter().map(|c| c.into()).collect(),
                groups: groups.into_iter().map(|g| g.parse().unwrap()).collect(),
            },
        ))
    }

    fn expand_by(&self, expansion: usize) -> Record {
        let springs = self.springs
                        .iter()
                        .cloned()
                        .chain([SpringType::Unknown].iter().cloned())
                        .cycle()
                        .take((self.springs.len() * expansion) + 4)
                        .collect();
        let groups = self.groups
                        .iter()
                        .cloned()
                        .cycle()
                        .take(self.groups.len() * expansion)
                        .collect();
        Record::new(springs, groups)
    }
}

#[derive(Debug)]
pub struct Day12;
impl Solution for Day12 {
    type ParsedInput = Vec<Record>;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        input_lines
            .lines()
            .map(|s| Record::parse_from_line(s).unwrap().1)
            .collect::<Vec<Record>>()
    }

    fn part_one(records: &mut Self::ParsedInput) -> String {
        let mut cache = HashMap::new();
        let solutions = records
            .iter()
            .map(|r| find_possible_solutions(&mut cache, r))
            .sum::<usize>();
        solutions.to_string()
    }

    fn part_two(records: &mut Self::ParsedInput) -> String {
        let mut cache = HashMap::new();
        let solutions = records
            .iter()
            .map(|r| find_possible_solutions(&mut cache, &r.expand_by(5)))
            .sum::<usize>();
        solutions.to_string()
    }
}

// DP, Recusrive with memo/cache to keep track of previously found solutions
// S# are the sentinel cases
fn find_possible_solutions(cache: &mut HashMap<Record, usize>, record: &Record) -> usize {
    use SpringType::*;
    // S1: we have already found a possible solution for the record
    if let Some(&v) = cache.get(record) {
        return v;
    }

    // S2: no groups left, there are no other damaged springs otherwise it cannot be valid
    if record.groups.is_empty() {
        let v = match record.springs.iter().any(|c| *c == Damaged) {
            true => 0,
            false => 1,
        };
        cache.insert(record.clone(), v);
        return v;
    }

    // S3: we have some groups left so ensure we have enough springs to fill them
    if record.springs.len() < record.groups.iter().sum::<usize>() + record.groups.len() - 1 {
        cache.insert(record.clone(), 0);
        return 0;
    }

    // S4: we cannot work with operational springs i.e. c == '.' so skip
    if record.springs[0] == Operational{
        let solutions = find_possible_solutions(
            cache,
            &Record::new(record.springs[1..].to_vec(), record.groups.clone()),
        );
        cache.insert(record.clone(), solutions);
        return solutions;
    } 

    // Here, we know we are at the beginning of a possible position for the current group. 
    // Check if that's possible and if it is, we find many valid solutions we'd get if we did.
    let mut solutions = 0;
    let cur = record.groups[0];
    let non_operational = record.springs[0..cur]
        .iter()
        .all(|c| *c != Operational);
    let end = (cur + 1).min(record.springs.len());
    if non_operational
        && ((record.springs.len() > cur && record.springs[cur] != Damaged)
            || record.springs.len() <= cur)
    {
        solutions = find_possible_solutions(
            cache,
            &Record::new(record.springs[end..].to_vec(), record.groups[1..].to_vec()),
        );
    }

    // If current position is Unknown, we can choose not to use that solution
    if record.springs[0] == Unknown {
        solutions += find_possible_solutions(
            cache,
            &Record::new(record.springs[1..].to_vec(), record.groups.clone()),
        );
    }

    // We have the number of solutions for this record, so cache and return it
    cache.insert(record.clone(), solutions);
    solutions
}