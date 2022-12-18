use clap::Parser;
use std::{fs::{read_to_string}, path::Path, process, time::Instant};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    day: u8
}

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod problem;

fn day_to_problem(day: u8) -> Option<Box<dyn problem::Problem>> {
    match day {
        1 => Some(Box::new(day1::Day1 {})),
        2 => Some(Box::new(day2::Day2 {})),
        3 => Some(Box::new(day3::Day3 {})),
        4 => Some(Box::new(day4::Day4 {})),
        5 => Some(Box::new(day5::Day5 {})),
        6 => Some(Box::new(day6::Day6 {})),
        7 => Some(Box::new(day7::Day7 {})),
        8 => Some(Box::new(day8::Day8 {})),
        // I stopped trying to pre-optimise here as I overdid day 8
        9 => Some(Box::new(day9::Day9 {})),
        10 => Some(Box::new(day10::Day10 {})),
        11 => Some(Box::new(day11::Day11 {})),
        12 => Some(Box::new(day12::Day12 {})),
        13 => Some(Box::new(day13::Day13 {})),
        14 => Some(Box::new(day14::Day14 {})),
        15 => Some(Box::new(day15::Day15 {})),
        16 => Some(Box::new(day16::Day16 {})),
        // ...
        _ => None,
    }
}

fn input_to_string<'a>(day: &str, part: &str) -> String {
    let binding = Path::new("src").join(&day).join(&part);
    let combined = binding.as_path();
    let input = read_to_string(combined);

    match input {
        Ok(res) => return res,
        Err(_err) => {
            println!("Could not find path: {}", combined.display());
            process::exit(0);
        }
    }
}

fn main() {
    let args = Args::parse();
    let problem = day_to_problem(args.day);

    let binding = format!("day{}", args.day);
    let folder = binding.as_str();

    let part1 = input_to_string(folder, "1.txt");
    let part2 = input_to_string(folder, "2.txt");

    match problem {
        Some(problem) => {
            let mut now = Instant::now();
            println!("Part 1: {}, Elapsed: {:.2?}", problem.part_one(&part1), now.elapsed());

            now = Instant::now();
            println!("Part 2: {}, Elapsed: {:.2?}", problem.part_two(&part2), now.elapsed());
        },
        None => println!("Invalid day specified")
    }
}
