use std::fmt::Debug;

use crate::problem::Problem;

pub struct Day14 {}

struct Point {
    x: usize,
    y: usize
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        return Point { x, y };
    }
}

impl Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "({}, {})", self.x, self.y);
    }
}

#[derive(PartialEq, Clone, Debug)]
enum Object {
    Sand,
    Air,
    Rock
}

impl Problem for Day14 {
    fn part_one(&self, input: &str) -> String {
        let mut grid = vec![vec![Object::Air; 1000]; 1000];
        let mut deepest_rock = 0;
        let mut sand_count = &mut 0;

        let rock_corners = input
            .lines()
            .map(|line| line
                .split(" -> ")
                .map(|coord| {
                    let mut s = coord.split(",");
                    let point = Point::new(s.next().unwrap().parse::<usize>().unwrap(), s.next().unwrap().parse::<usize>().unwrap());
                    if point.y > deepest_rock { deepest_rock = point.y; }
                    return point;
                })
                .collect::<Vec<Point>>()
            )
            .collect::<Vec<Vec<Point>>>();

        build_starting_grid(&rock_corners, &mut grid);
        place_sand_1(&mut grid, deepest_rock, &mut sand_count);

        return format!("{}", sand_count);
    }

    fn part_two(&self, input: &str) -> String {
        let mut deepest_rock = 0;
        let mut sand_count = &mut 0;

        let rock_corners = input
            .lines()
            .map(|line| line
                .split(" -> ")
                .map(|coord| {
                    let mut s = coord.split(",");
                    let point = Point::new(s.next().unwrap().parse::<usize>().unwrap(), s.next().unwrap().parse::<usize>().unwrap());
                    if point.y > deepest_rock { deepest_rock = point.y; }
                    return point;
                })
                .collect::<Vec<Point>>()
            )
            .collect::<Vec<Vec<Point>>>();

        // Lay the bottom layer
        let mut grid = vec![vec![Object::Air; 1000]; deepest_rock+3];
        for i in 0..grid[0].len() {
            grid[deepest_rock+2][i] = Object::Rock;
        }

        build_starting_grid(&rock_corners, &mut grid);
        place_sand_2(&mut grid, &mut sand_count);

        return format!("{}", sand_count);
    }
}

fn place_sand_2(grid: &mut Vec<Vec<Object>>, sand_count: &mut usize) {
    loop {
        let mut sand_pos = Point::new(500, 0);

        loop {
            // Down
            while grid[sand_pos.y+1][sand_pos.x] == Object::Air {
                sand_pos.y += 1;
            }
            // Down-Left
            if grid[sand_pos.y+1][sand_pos.x-1] == Object::Air {
                sand_pos.y += 1;
                sand_pos.x -= 1;
                continue;
            }
            // Down-Right
            if grid[sand_pos.y+1][sand_pos.x+1] == Object::Air {
                sand_pos.y += 1;
                sand_pos.x += 1;
                continue;
            }
            break;
        }
        grid[sand_pos.y][sand_pos.x] = Object::Sand;
        *sand_count += 1;

        if sand_pos.y == 0 { return; }
    }
}

fn place_sand_1(grid: &mut Vec<Vec<Object>>, deepest_rock: usize, sand_count: &mut usize) {
    loop {
        let mut sand_pos = Point::new(500, 0);

        loop {
            // Gone too deep, so done
            if sand_pos.y >= deepest_rock { return; }

            // Down
            while grid[sand_pos.y+1][sand_pos.x] == Object::Air {
                sand_pos.y += 1;
                // Gone too deep, so done
                if sand_pos.y >= deepest_rock { return; }
            }
            // Down-Left
            if grid[sand_pos.y+1][sand_pos.x-1] == Object::Air {
                sand_pos.y += 1;
                sand_pos.x -= 1;
                continue;
            }
            // Down-Right
            if grid[sand_pos.y+1][sand_pos.x+1] == Object::Air {
                sand_pos.y += 1;
                sand_pos.x += 1;
                continue;
            }
            break;
        }
        grid[sand_pos.y][sand_pos.x] = Object::Sand;
        *sand_count += 1;
    }
}

fn build_starting_grid(rock_corners: &Vec<Vec<Point>>, grid: &mut Vec<Vec<Object>>) {
    for path in rock_corners.iter() {
        for (point_i, point) in path.iter().enumerate() {
            if point_i == path.len() - 1 { break; }

            let next_point = &path[point_i + 1];
            // Should not be able to move diagonally
            assert!(point.x == next_point.x || point.y == next_point.y);

            // Rust can't iterate backwards...
            let low_x = if point.x > next_point.x { next_point.x } else { point.x };
            let high_x = if point.x > next_point.x { point.x } else { next_point.x };

            let low_y = if point.y > next_point.y { next_point.y } else { point.y };
            let high_y = if point.y > next_point.y { point.y } else { next_point.y };

            for x in low_x..=high_x {
                for y in low_y..=high_y {
                    grid[y][x] = Object::Rock;
                }
            }
        }
    }
}
