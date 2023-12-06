use aoc2023::solve_day;
use clap::Parser;

#[derive(Parser)]
#[command(author="Shaan Arora", version="0.1.0", about="Advent of Code 2023", long_about="None")]
struct Cli {
    day: Option<i32>
}

fn main() {
    let cli = Cli::parse();
    let mut days = (0..=25).collect::<Vec<i32>>();
    let mut days_to_execute = vec![];
    if let Some(day) = cli.day {
        if !days.contains(&day) {
            panic!("Day not found!");
        }

        days_to_execute.push(day);
    } else {
        days_to_execute = days.drain(1..).collect(); //skips day 0 example
    }

    for day in days_to_execute {
        solve_day(&day);
    }
}
