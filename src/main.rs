use clap::Parser;
use day1::Day1;
use problem::Problem;
use std::{fs::{read_to_string, read_dir}, path::Path, process};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    day: u8
}

mod day1;
mod problem;

fn day_to_problem(day: u8) -> Option<Box<dyn Problem>> {
    match day {
        1 => Some(Box::new(Day1 {})),
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
            println!("{}", problem.part_one(&part1));
            println!("{}", problem.part_two(&part2));
        },
        None => println!("Invalid day specified")
    }
}
