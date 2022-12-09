use std::ops::Range;

use crate::problem::Problem;

pub struct Day8 {}

impl Problem for Day8 {
    fn part_one(&self, input: &str) -> String {
        let grid = input
            .lines()
            .map(|l| l
                .split("")
                .filter(|num| num.len() != 0)
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<i32>>())
            .collect::<Vec<Vec<i32>>>();

        // Grid of truths for which trees can be seen (0: false, 1: true)
        let mut truth = vec![vec![0; grid[0].len()]; grid.len()];

        // View from West
        for i in 0..grid.len() {
            let mut curr_max = -1;
            for j in 0..grid[i].len() {
                let height = grid[i][j];

                if height > curr_max {
                    truth[i][j] = 1;
                    curr_max = height;
                }
            }
        }

        // View from East
        for i in 0..grid.len() {
            let mut curr_max = -1;
            for j in (0..grid[i].len()).rev() {
                let height = grid[i][j];

                if height > curr_max {
                    truth[i][j] = 1;
                    curr_max = height;
                }
            }
        }

        // View from North
        update_truth_vertical(&grid, &mut truth, Vec::from_iter(0..grid.len()));

        // View from South
        update_truth_vertical(&grid, &mut truth, Vec::from_iter((0..grid.len()).rev()));


        return format!("{}", truth.iter().map(|row| row.iter().sum::<i32>()).sum::<i32>());
    }

    fn part_two(&self, input: &str) -> String {
        let grid = input
            .lines()
            .map(|l| l
                .split("")
                .filter(|num| num.len() != 0)
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<i32>>())
            .collect::<Vec<Vec<i32>>>();

        let peaks = get_peaks(&grid);
        let scores = peaks.iter().map(|p| calculate_point_score(&grid, p));

        return format!("{:?}", scores.max().unwrap());
    }
}

// point (j, i) in this form for printing purposes
fn calculate_point_score(grid: &Vec<Vec<i32>>, point: &(usize, usize)) -> usize {
    let val = grid[point.1][point.0];

    let mut north_score = 0;
    for y in (0..point.1).rev() {
        north_score += 1;
        if grid[y][point.0] >= val { break; }
    }
    let mut south_score = 0;
    for y in point.1+1..grid.len() {
        south_score += 1;
        if grid[y][point.0] >= val { break; }
    }

    let mut west_score = 0;
    for x in (0..point.0).rev() {
        west_score += 1;
        if grid[point.1][x] >= val { break; }
    }
    let mut east_score = 0;
    for x in point.0+1..grid[0].len() {
        east_score += 1;
        if grid[point.1][x] >= val { break; }
    }

    return east_score * west_score * north_score * south_score;
}

// If we have the row/column peaks, we know that nothing can be higher
// This means we can ignore the vast majority of the computations for irrelevant points
fn get_peaks(grid: &Vec<Vec<i32>>) -> Vec::<(usize, usize)> {
    let mut peaks = Vec::<(usize, usize)>::new();
    let mut max_cols = vec![0; grid[0].len()];
    let mut max_rows = vec![0; grid.len()];

    // Finding the maximums of each row/column combo
    // We ignore the outer perimeter
    for y in 1..max_rows.len()-1 {
        for x in 1..max_cols.len()-1 {
            let height = grid[y][x];

            if height > max_cols[x] { max_cols[x] = height; }
            if height > max_rows[y] { max_rows[y] = height; }
        }
    }

    // Finding the peak coords so we can ignore every other point
    // We ignore the outer perimeter
    for y in 1..max_rows.len()-1 {
        for x in 1..max_cols.len()-1 {
            if grid[y][x] == max_cols[x] || grid[y][x] == max_rows[y] { peaks.push((x, y)); }
        }
    }

    return peaks;
}


// CBA to do for horizontal, just showing that it can be extracted if wanted
fn update_truth_vertical(grid: &Vec<Vec<i32>>, truth: &mut Vec<Vec<i32>>, iter: Vec<usize>) {
    for j in 0..grid[0].len() {
        let mut curr_max = -1;
        for &i in iter.iter() {
            let height = grid[i][j];

            if height > curr_max {
                truth[i][j] = 1;
                curr_max = height;
            }
        }
    }
}
