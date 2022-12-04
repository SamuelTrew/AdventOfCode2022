use std::{collections::HashSet};

use crate::problem::Problem;

pub struct Day3 {}

impl Problem for Day3 {
    fn part_one(&self, input: &str) -> String {
        let rucksacks = input.split("\n");
        let mut res = 0;
        let mut fst = HashSet::<char>::new();

        for sack in rucksacks {
            let mut chars = sack.chars();
            let div = sack.len() / 2;
            fst.clear();

            // For some reason nth(0) consumes the value
            for _ in 0..div {
                fst.insert(chars.nth(0).unwrap());
            }

            for _ in div..sack.len() {
                let common = chars.nth(0).unwrap();
                if fst.contains(&common) {
                    res += char_convert(&common);
                    break;
                }
            }
        }

        return format!("{}", res);
    }

    fn part_two(&self, input: &str) -> String {
        let mut rucksacks = input.split("\n");
        let mut res = 0;

        while let Some(one) = rucksacks.next() {
            let fst = HashSet::<char>::from_iter(one.chars());
            let snd = HashSet::<char>::from_iter(rucksacks.next().unwrap().chars());
            let thr = HashSet::<char>::from_iter(rucksacks.next().unwrap().chars());

            // '\r' is added when intersecting
            let common = fst.intersection(&snd).filter(|c| **c != '\r');

            for val in common {
                if thr.contains(val) {
                    res += char_convert(&val);
                    break;
                }
            }
        }

        return format!("{}", res);
    }
}

// Converts character to question point representation
// a-z = 1-26
// A-Z = 27-52
fn char_convert(c: &char) -> i32 {
    let ord = *c as i32;
    if ord <= 90 { return ord - 38; }

    return ord - 96;
}
