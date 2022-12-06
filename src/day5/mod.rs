use crate::problem::Problem;

pub struct Day5 {}

impl Problem for Day5 {
    fn part_one(&self, input: &str) -> String {
        let (mut stacks, mut moves) = parse(input);

        while let Some((count, from, to)) = moves.next() {
            for _ in 0..count {
                let c = stacks[from].pop().unwrap();
                stacks[to].push(c);
            }
        }

        let res = stacks.iter().map(|stack| stack.last().unwrap()).collect::<String>();
        return format!("{:?}", res);
    }

    fn part_two(&self, input: &str) -> String {
        let (mut stacks, mut moves) = parse(input);

        while let Some((count, from, to)) = moves.next() {
            let idx = stacks[from].len() - count;
            let mut chrs = stacks[from].split_off(idx);
            stacks[to].append(&mut chrs);
        }

        let res = stacks.iter().map(|stack| stack.last().unwrap()).collect::<String>();
        return format!("{:?}", res);
    }
}

fn parse(input: &str) -> (Vec<Vec<char>>, impl Iterator<Item = (usize, usize, usize)> + '_) {
    let mut sections = input.split("\n\n");
    let stacks = parse_stacks(sections.next().unwrap());
    let moves = parse_moves(sections.next().unwrap());

    return (stacks, moves)
}

fn parse_stacks(input: &str) -> Vec<Vec<char>> {
    let mut rows = input.split('\n').rev();
    let bottom = rows.next().unwrap();
    let stack_count = bottom.chars().rev().nth(1).unwrap().to_digit(10).unwrap().try_into().unwrap();

    let mut stacks = vec![Vec::<char>::new(); stack_count];

    // row: "[A] [B]    "
    // row: "    [X] [Y]"
    // row: "[Z] [M] [P]"
    while let Some(row) = rows.next() {
        let mut chars = row.chars().skip(1); // Skips the first "["
        for i in 0..stack_count {
            let c = chars.next().unwrap();
            chars.nth(2); // Skips the "] ["
            if c != ' ' { stacks[i].push(c) }; // Whitespace means no more data for that stack
        }
    }

    return stacks;
}

// result: (count, fromStack, toStack)
fn parse_moves(input: &str) -> impl Iterator<Item = (usize, usize, usize)> + '_ {
    return input
        .lines()
        .map(|mv| mv.split(' '))
        .map(|mut mv| (
            mv.nth(1).unwrap().parse::<usize>().unwrap(),
            mv.nth(1).unwrap().parse::<usize>().unwrap() - 1, // Converting number to index
            mv.nth(1).unwrap().parse::<usize>().unwrap() - 1
        ));
}
