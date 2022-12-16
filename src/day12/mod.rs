use std::collections::{VecDeque};

use crate::problem::Problem;

pub struct Day12 {}

impl Problem for Day12 {
    fn part_one(&self, input: &str) -> String {
        let mut start = (0, 0); // This should not be the case unless S actually is (0, 0)
        let grid = input
            .lines()
            .enumerate()
            .map(|(y, l)| l
                .chars()
                .filter(|chr| *chr != '\r')
                .enumerate()
                .map(|(x, chr)| {
                    if chr == 'S' { start = (x, y);  return 0; }
                    if chr == 'E' { return 1000; }
                    return chr as i32 - 97;
                }) // Turn to numbers
                .collect::<Vec<i32>>())
            .collect::<Vec<Vec<i32>>>();

        let result: &mut Vec<Vec<Option<i32>>> = &mut vec![vec![None; grid[0].len()]; grid.len()];
        result[start.1][start.0] = Some(0);

        bfs(&grid, result, start);

        return format!("{:?}", result.iter().map(|l| l.iter().filter(|res| res.is_some()).map(|res| res.unwrap()).max().unwrap()).max().unwrap());
    }

    fn part_two(&self, input: &str) -> String {
        return format!("Part 1 not implemented.");
    }
}

// We care if the next door results are greater, if so overwrite them
// This means that it has taken longer to reach that point than the current iteration
// If they are None then we haven't visited and we continue
fn bfs (grid: &Vec<Vec<i32>>, result: &mut Vec<Vec<Option<i32>>>, point: (usize, usize)) {
    let mut queue = VecDeque::from([point]);

    while let Some((x, y)) = queue.pop_front() {
        let curr_depth = grid[y][x];
        let curr_result = result[y][x].unwrap();

        // Left
        if x != 0 {
            search_in_direction(grid, result, x-1, y, curr_depth, curr_result, &mut queue);
        }

        // Right
        if x != grid[0].len()-1 {
            search_in_direction(grid, result, x+1, y, curr_depth, curr_result, &mut queue);
        }

        // Up
        if y != 0 {
            search_in_direction(grid, result, x, y-1, curr_depth, curr_result, &mut queue)
        }

        // Down
        if y != grid.len()-1 {
            search_in_direction(grid, result, x, y+1, curr_depth, curr_result, &mut queue)
        }
    }
}

fn search_in_direction(grid: &Vec<Vec<i32>>, result: &mut Vec<Vec<Option<i32>>>, x: usize, y: usize, curr_depth: i32, curr_result: i32, queue: &mut VecDeque<(usize, usize)>) {
    let depth = grid[y][x];

    if depth == 1000 { // E
        result[y][x] = Some(curr_result + 1);
        return;
    }

    let res = result[y][x];
    let can_go =
        // 1 higher level or same or lower
        depth <= curr_depth + 1 &&
        // Left hasn't been visited or left result took longer to reach that point than the current iteration
        (res == None ||
        res.unwrap() > curr_result + 1);

    if can_go {
        result[y][x] = Some(curr_result + 1);
        queue.push_back((x, y));
    }
}