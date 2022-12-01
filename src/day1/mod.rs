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
        let mut total_food = elves.map(|elf| elf.split("\n").map(|food| food.parse::<i32>().unwrap()).sum::<i32>());

        let mut first = 0;
        let mut second = 0;
        let mut third = 0;

        while let Some(food) = total_food.next() {
            if food > first {
                third = second;
                second = first;
                first = food;
            } else if food > second {
                third = second;
                second = food;
            } else if food > third {
                third = food;
            }
        }

        return format!("{}", first + second + third);
    }
}
