use std::process::exit;

use crate::problem::Problem;

pub struct Day2 {}

impl Problem for Day2 {
    fn part_one(&self, input: &str) -> String {
        let game = input.split("\n");
        let pairs = game
            .map(|game|(game.chars().nth(0).unwrap(), game.chars().nth(2).unwrap()))
            .map(calc_score);

        return format!("{}", pairs.sum::<i32>());
    }

    fn part_two(&self, input: &str) -> String {
        let game = input.split("\n");
        let pairs = game
            .map(|game|(game.chars().nth(0).unwrap(), game.chars().nth(2).unwrap()))
            .map(calc_move_pair)
            .map(calc_score);

        return format!("{}", pairs.sum::<i32>());
    }
}

// me-opp   result  +4 %3
// 2        lose    0
// 1        win     2
// 0        draw    1
// -1       lose    0
// -2       win     2

fn calc_score(pair: (char, char)) -> i32 {
    let opp = convert_char(pair.0);
    let me = convert_char(pair.1);

    // Short circuit
    if me == opp { return 3 + me; }

    let diff = me-opp;

    return (((diff+4) % 3) * 3) + me;
}

fn calc_move_pair(pair: (char, char)) -> (char, char) {
    let opp = convert_char(pair.0);

    return (pair.0, match pair.1 {
        'X' => convert_code((opp + 2) % 3),
        'Y' => pair.0,
        'Z' => convert_code((opp + 4) % 3),
        _ => exit(1)
    });
}

fn convert_code(c: i32) -> char {
    match c {
        1 => 'A',
        2 => 'B',
        0 => 'C', // Essentially 0 is 3 in this case
        _ => exit(1)
    }
}

fn convert_char(c: char) -> i32 {
    match c {
        'A' => 1,
        'B' => 2,
        'C' => 3,
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => exit(1),
    }
}
