use crate::problem::Problem;

pub struct Day8 {}

impl Problem for Day8 {
    fn part_one(&self, input: &str) -> String {
        let grid = input
            .lines()
            .map(|l| l
                .split("")
                .filter(|num| num.len() != 0)
                .map(|num| num.parse::<u32>().unwrap())
                .collect::<Vec<u32>>())
            .collect::<Vec<Vec<u32>>>();

        for i in 1..grid.len() - 1 {
            for j in 1..grid.len() - 1 {
                println!("{}", grid[i][j]);
            }
        }
        return format!("Part 1 not implemented.");
    }

    fn part_two(&self, input: &str) -> String {
        return format!("Part 1 not implemented.");
    }
}
