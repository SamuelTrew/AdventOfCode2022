use crate::problem::Problem;

pub struct Day1 {}

impl Problem for Day1 {
    fn part_one(&self, input: &str) -> String {
        let elves = input.split("\n\n");
        let total_food = elves.map(|elf| elf.split("\n").map(|food| food.parse::<i32>().unwrap()).sum::<i32>());

        return format!("{:?}", total_food.max());
    }

    fn part_two(&self, input: &str) -> String {
        let elves = input.split("\n\n");
        let mut total_food = elves
            .map(|elf| elf.split("\n")
                .map(|food| food.parse::<i32>().unwrap())
                .sum::<i32>())
            .collect::<Vec<i32>>();

        total_food.sort_by(|a, b| b.cmp(a));

        return format!("{}", total_food[0] + total_food[1] + total_food[2]);
    }
}
