use crate::problem::Problem;

#[derive(Debug, Clone)]
struct Monkey<'a> {
    items: Vec<i64>,
    operation: Vec<&'a str>,
    test_num: i64,
    test_pos: usize,
    test_neg: usize,
}

impl<'a> Monkey<'a> {
    fn update_item(&mut self, i: usize) {
        let op = self.operation[0];
        let val = self.operation[1];

        let item = self.items[i];
        let mut new_item;

        if val == "old" {
            new_item = if op == "*" { item * item } else { item + item };
        } else {
            let num = val.parse::<i64>().unwrap();
            new_item = if op == "*" { item * num } else { item + num };
        }

        new_item /= 3;
        self.items[i] = new_item;
    }

    // No longer modding by 3 means numbers explode and so we have to adjust them
    fn update_item_stress(&mut self, i: usize, lowest_common_multiple: i64) {
        let op = self.operation[0];
        let val = self.operation[1];

        let item = self.items[i];
        let mut new_item;

        if val == "old" {
            new_item = if op == "*" { item * item } else { item + item };
        } else {
            let num = val.parse::<i64>().unwrap();
            new_item = if op == "*" { item * num } else { item + num };
        }

        // If we use the lcm for modding then all monkeys will be affected in the same way
        // Can just multiply all divisors together as well and that would work (might be too large)
        new_item %= lowest_common_multiple;
        if new_item == 0 { new_item = lowest_common_multiple; }
        self.items[i] = new_item;
    }
}

pub struct Day11 {}

impl Problem for Day11 {
    fn part_one(&self, input: &str) -> String {
        let temp = input.replace('\r', "");
        let filtered = temp.as_str();
        let monkeys = &mut get_monkeys(filtered);
        let mut inspections = vec![0; monkeys.len()];

        for _ in 0..20 {
            for i in 0..monkeys.len() {
                let items = &mut monkeys[i].items;
                for j in (0..items.len()).rev() {
                    monkeys[i].update_item(j);
                    inspections[i] += 1;

                    let can_div = monkeys[i].items[j] % monkeys[i].test_num == 0;
                    let new_monkey = if can_div { monkeys[i].test_pos } else { monkeys[i].test_neg };
                    let item = monkeys[i].items.pop().unwrap();
                    monkeys[new_monkey].items.push(item);
                }
            }
        }

        let mut one = 0;
        let mut two = 0;

        for count in inspections {
            if count > one {
                two = one;
                one = count;
            } else if count > two {
                two = count;
            }
        }

        return format!("{:?}", one * two);
    }

    fn part_two(&self, input: &str) -> String {
        let temp = input.replace('\r', "");
        let filtered = temp.as_str();
        let monkeys = &mut get_monkeys(filtered);
        let mut inspections = vec![0; monkeys.len()];
        let mut lowest_common_multiple = monkeys[0].test_num;

        for i in 1..monkeys.len() {
            lowest_common_multiple = lcm(lowest_common_multiple, monkeys[i].test_num);
        }

        for _ in 0..10_000 {
            for i in 0..monkeys.len() {
                let items = &mut monkeys[i].items;
                for j in (0..items.len()).rev() {
                    monkeys[i].update_item_stress(j, lowest_common_multiple);
                    inspections[i] += 1;

                    let can_div = monkeys[i].items[j] % monkeys[i].test_num == 0;
                    let new_monkey = if can_div { monkeys[i].test_pos } else { monkeys[i].test_neg };
                    let item = monkeys[i].items.pop().unwrap();
                    monkeys[new_monkey].items.push(item);
                }
            }
        }

        let mut one: i64 = 0;
        let mut two: i64 = 0;

        for count in inspections {
            if count > one {
                two = one;
                one = count;
            } else if count > two {
                two = count;
            }
        }

        return format!("{:?}", one * two);
    }
}

fn get_monkeys(input: &str) -> Vec<Monkey> {
    let mut sections = input.split("\n\n");
    let mut res = Vec::<Monkey>::new();

    while let Some(str_sec) = sections.next() {
        let sec = &mut str_sec.split("\n").skip(1);

        let items = sec
            .next().unwrap() // Getting items line
            .split(": ")
            .last().unwrap() // Removing boilerplate text
            .split(", ")
            .map(|num| num.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        let operation = sec
            .next().unwrap() // Getting operation line
            .split(" = ")
            .last().unwrap()
            .split(" ").skip(1)
            .collect::<Vec<&str>>();

        let test_num = sec
            .next().unwrap()
            .split(" ")
            .last().unwrap()
            .parse::<i64>().unwrap();

        let test_pos = sec
            .next().unwrap()
            .split(" ")
            .last().unwrap()
            .parse::<usize>().unwrap();

        let test_neg = sec
            .next().unwrap()
            .split(" ")
            .last().unwrap()
            .parse::<usize>().unwrap();

        res.push(Monkey {
            items,
            operation,
            test_num,
            test_pos,
            test_neg
        });
    }

    return res;
}

// Return greatest common divisor using Euclid's Algorithm.
fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let rem = a % b;
        a = b;
        b = rem;
    }
    return a;
}

// Return lowest common multiple
fn lcm(a: i64, b: i64) -> i64 {
    return a * b / gcd(a, b);
}
