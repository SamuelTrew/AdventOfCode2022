use std::collections::HashMap;

use crate::problem::Problem;

pub struct Day7 {}

impl Problem for Day7 {
    fn part_one(&self, input: &str) -> String {
        let mut child_points = HashMap::<char, i32>::new();
        // Why build a tree when you can have a self-referencing HashMap :)
        let mut dir_references = HashMap::<char, Vec::<char>>::new();

        let mut cmds = input.lines().filter(|l| *l != "$ cd ..");
        let mut dir = '?';

        // Build refs and head points
        while let Some(cmd) = cmds.next() {
            if cmd.contains("$ cd ") {
                dir = cmd.chars().last().unwrap();
                let ls = cmds.next().unwrap();
                assert!(ls == "$ ls");
                continue;
            }

            let mut info = cmd.split(" ");
            let ref_or_val = info.next().unwrap();
            if ref_or_val == "dir" {
                let empty_vec = &Vec::<char>::new();
                let mut refs = dir_references.get(&dir).unwrap_or(empty_vec).to_owned();
                let new_char = cmd.chars().last().unwrap();

                refs.push(new_char);

                dir_references.insert(dir, refs);
                continue;
            }

            let mut curr_val = child_points.get(&dir).unwrap_or(&0).to_owned();
            curr_val += ref_or_val.parse::<i32>().unwrap();

            child_points.insert(dir, curr_val);
        }

        // Compact points
        let res = child_points
            .iter()
            .map(|(dir, val)| val + compact(&child_points, &dir_references, dir))
            .filter(|val| *val <= 100000)
            .sum::<i32>();

        return format!("{}", res);
    }

    fn part_two(&self, input: &str) -> String {
        return format!("Part 1 not implemented.");
    }
}

// I know that this will iterate over the whole "tree" each time, slight inefficiency on my part (stack overflow)
fn compact(points: &HashMap::<char, i32>, map: &HashMap::<char, Vec::<char>>, dir: &char) -> i32 {
    let refs = map.get(dir);

    match refs {
        Some(r) => return r.iter().map(|d| points.get(d).unwrap() + compact(points, map, d)).sum::<i32>(),
        None => return 0,
    };
}
