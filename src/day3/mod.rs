use std::{collections::HashSet};
use asciis::asc::Asciis;

use crate::problem::Problem;

pub struct Day3 {}

impl Problem for Day3 {
    fn part_one(&self, input: &str) -> String {
        let rucksacks = input.split("\n");
        let mut common_chars = Vec::new();

        for sack in rucksacks {
            let mut chars = sack.chars();
            let div = sack.len() / 2;

            let mut fst = HashSet::<char>::new();

            // For some reason nth(0) consumes the value
            for _ in 0..div {
                fst.insert(chars.nth(0).unwrap());
            }

            for _ in div..sack.len() {
                let common = chars.nth(0).unwrap();
                if fst.contains(&common) {
                    common_chars.push(common);
                    break;
                }
            }
        }

        return format!("{}", common_chars.iter().map(char_convert).sum::<i32>());
    }

    fn part_two(&self, input: &str) -> String {
        let mut rucksacks = input.split("\n");
        let mut common_chars = Vec::new();

        while let Some(one) = rucksacks.next() {
            let fst = HashSet::<char>::from_iter(one.chars());
            let snd = HashSet::<char>::from_iter(rucksacks.next().unwrap().chars());
            let thr = HashSet::<char>::from_iter(rucksacks.next().unwrap().chars());

            // '\r' is added when intersecting
            let common = fst.intersection(&snd).filter(|c| **c != '\r');

            for val in common {
                if thr.contains(val) {
                    common_chars.push(val.clone());
                    break;
                }
            }
        }

        return format!("{}", common_chars.iter().map(char_convert).sum::<i32>());
    }
}

// Converts character to question point representation
// a-z = 1-26
// A-Z = 27-52
fn char_convert(c: &char) -> i32 {
    let asc = Asciis{};
    let val = asc.ord(c.encode_utf8(&mut [0; 4])).unwrap();

    if val <= 90 { return val - 38; }

    return val - 96;
}
