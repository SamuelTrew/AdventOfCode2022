use std::collections::{HashSet, HashMap};

use crate::problem::Problem;

pub struct Day6 {}

impl Problem for Day6 {
    fn part_one(&self, input: &str) -> String {
        const SIZE: usize = 4;
        let mut chars = input.chars().enumerate();
        let mut buff = [chars.next().unwrap().1; SIZE];
        let mut ptr = 1;

        // I wrote one that doesn't re-create a HashSet each time but was only 1ms faster and 4 times as long
        while let Some((i, c)) = chars.next() {
            if HashSet::from(buff).len() == SIZE { return format!("{}", i)}
            buff[ptr] = c;
            ptr = (1+ptr) % SIZE;
        }

        return format!("Part 1 failed to be implemented correctly");
    }

    fn part_two(&self, input: &str) -> String {
        const SIZE: usize = 14;
        let mut chars = input.chars().enumerate();
        let first = chars.next().unwrap().1;
        let mut buff = [first; SIZE];
        let mut ptr = 1;
        let mut counter = HashMap::from([(first, SIZE)]);

        // Nvm used this implementation here to get ~8x speed-up
        while let Some((i, c)) = chars.next() {
            if counter.len() == SIZE { return format!("{}", i)}

            let prev = buff[ptr];
            let val = counter.get(&prev).unwrap(); // We "know" it'll exist
            if *val == 1 {
                counter.remove(&prev);
            } else {
                counter.insert(prev, *val-1);
            }

            let new_val = counter.get(&c);
            match new_val {
                Some(v) => counter.insert(c, *v+1),
                None => counter.insert(c, 1)
            };

            buff[ptr] = c;
            ptr = (1+ptr) % SIZE;
        }

        return format!("Part 2 failed to be implemented correctly");
    }
}
