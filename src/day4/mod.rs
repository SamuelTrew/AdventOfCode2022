use crate::problem::Problem;

pub struct Day4 {}

impl Problem for Day4 {
    fn part_one(&self, input: &str) -> String {
        let pairs = get_pairs(input);

        let count = pairs
            .map(|p| {
                let left_contains = p.0.0 <= p.1.0 && p.0.1 >= p.1.1; // 25-87,26-55 => true
                let right_contains = p.0.0 >= p.1.0 && p.0.1 <= p.1.1; // 26-55,25-87 => true

                return (left_contains || right_contains) as i32;
            })
            .sum::<i32>();

        return format!("{}", count);
    }

    fn part_two(&self, input: &str) -> String {
        let pairs = get_pairs(input);

        let count = pairs
            .map(|p| {
                let r_overlap = p.1.0 <= p.0.1 && p.1.1 >= p.0.0;
                let l_overlap = p.0.0 <= p.1.1 && p.0.1 >= p.1.0;

                return (r_overlap || l_overlap) as i32;
            })
            .sum::<i32>();

        return format!("{}", count);
    }
}

fn get_pairs(input: &str) -> impl Iterator<Item = ((i32, i32), (i32, i32))> + '_ {
    return input
        .split("\n")
        .map(|elf_pair| elf_pair
            .split(&['-', ',', '\r'][..]) // 25-87,25-25 => [25, 87, 25, 25]
        )
        .map(|mut pair|
            (
                (pair.next().unwrap().parse::<i32>().unwrap(), pair.next().unwrap().parse::<i32>().unwrap()),
                (pair.next().unwrap().parse::<i32>().unwrap(), pair.next().unwrap().parse::<i32>().unwrap()),
            )
        );
}
