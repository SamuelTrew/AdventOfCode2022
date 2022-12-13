use std::collections::VecDeque;

use crate::problem::Problem;

pub struct Day12 {}

impl Problem for Day12 {
    fn part_one(&self, input: &str) -> String {
        let mut start = (0, 0); // This should not be the case unless S actually is (0, 0)
        let grid = input
            .lines()
            .enumerate()
            .map(|(i, l)| l
                .chars()
                .filter(|chr| *chr != '\r')
                .enumerate()
                .map(|(j, chr)| {
                    if chr == 'S' { start = (i, j);  return 0; }
                    if chr == 'E' { return 26; }
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
    let mut queue = VecDeque::<(usize, usize)>::from([point]);
    let mut visited = 0;
    let mut max_depth = 0;

    while let Some((x, y)) = queue.pop_front() {
        visited += 1;
        let curr_depth = grid[y][x];
        let curr_result = result[y][x].unwrap();

        if max_depth < curr_depth { max_depth = curr_depth; }

        // Left
        if x != 0 {
            let left_depth = grid[y][x-1];
            let left_result = result[y][x-1];
            let can_go_left =
                // 1 higher level or same or lower
                left_depth <= curr_depth + 1 &&
                // Left hasn't been visited or left result took longer to reach that point than the current iteration
                (left_result == None ||
                left_result.unwrap() > curr_result + 1);

            if can_go_left {
                if left_result != None {
                    println!("left{}", left_result.unwrap() - curr_result + 1);
                }
                result[y][x-1] = Some(curr_result + 1);
                println!("{:?}:{}", (x, y), curr_result + 1);
                queue.push_back((x-1, y));
            }
        }

        // Right
        if x != grid[0].len()-1 {
            let right_depth = grid[y][x+1];
            let right_result = result[y][x+1];
            let can_go_right =
                // Check level
                right_depth <= curr_depth + 1 &&
                // Check result
                (right_result == None ||
                right_result.unwrap() > curr_result + 1);

            if can_go_right {
                if right_result != None {
                    println!("right{}", right_result.unwrap() - curr_result + 1);
                }
                result[y][x+1] = Some(curr_result + 1);
                println!("{:?}:{}", (x, y), curr_result + 1);
                queue.push_back((x+1, y));
            }
        }

        // Up
        if y != 0 {
            let up_depth = grid[y-1][x];
            let up_result = result[y-1][x];
            let can_go_up =
                // Check level
                up_depth <= curr_depth + 1 &&
                // Check result
                (up_result == None ||
                up_result.unwrap() > curr_result + 1);

            if can_go_up {
                if up_result != None {
                    println!("up{}", up_result.unwrap() - curr_result + 1);
                }
                result[y-1][x] = Some(curr_result + 1);
                println!("{:?}:{}", (x, y), curr_result + 1);
                queue.push_back((x, y-1));
            }
        }

        // Down
        if y != grid.len()-1 {
            let down_depth = grid[y+1][x];
            let down_result = result[y+1][x];
            let can_go_down =
                // Check level
                down_depth <= curr_depth + 1 &&
                // Check result
                (down_result == None ||
                down_result.unwrap() > curr_result + 1);

            if can_go_down {
                if down_result != None {
                    println!("down{}", down_result.unwrap() - curr_result + 1);
                }
                result[y+1][x] = Some(curr_result + 1);
                println!("{:?}:{}", (x, y), curr_result + 1);
                queue.push_back((x, y+1));
            }
        }
    }
    println!("{}", visited);
    println!("{}", max_depth);
}
