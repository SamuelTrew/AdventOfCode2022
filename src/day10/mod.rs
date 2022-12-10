use crate::problem::Problem;

pub struct Day10 {}

impl Problem for Day10 {
    fn part_one(&self, input: &str) -> String {
        let mut cmds = input.lines();
        let mut x = 1;
        let mut res = 0;
        let mut cycle = 1;

        while let Some(cmd) = cmds.next() {
            if (cycle + 20) % 40 == 0 { res += x*cycle; }
            if cmd == "noop" { cycle += 1; continue; }

            if (cycle + 21) % 40 == 0 { res += x*(cycle+1); }

            let val = cmd.split(" ").last().unwrap().parse::<i32>().unwrap();
            x += val;
            cycle += 2;
        }

        return format!("{}", res);
    }

    fn part_two(&self, input: &str) -> String {
        let mut cmds = input.lines();
        let mut x = 1;
        let mut crt = "".to_owned();
        let mut cycle = 1;

        while let Some(cmd) = cmds.next() {
            let row_pos = (cycle-1) % 40;
            if row_pos == x || row_pos == x-1 || row_pos == x+1 {
                crt += "#";
            } else {
                crt += ".";
            }
            if cycle % 40 == 0 { crt += "\n"; }

            if cmd == "noop" { cycle += 1; continue; }

            let next_pos = cycle % 40;
            if next_pos == x || next_pos == x-1 || next_pos == x+1 {
                crt += "#";
            } else {
                crt += ".";
            }
            if (cycle+1) % 40 == 0 { crt += "\n"; }


            let val = cmd.split(" ").last().unwrap().parse::<i32>().unwrap();
            x += val;
            cycle += 2;
        }

        return format!("\n{}", crt);
    }
}
