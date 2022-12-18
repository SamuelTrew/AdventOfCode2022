use fnv::{FnvHasher, FnvHashSet};
use std::{fmt::Debug, collections::HashSet, hash::BuildHasherDefault};

use crate::problem::Problem;

pub struct Day15 {}

struct Sensor1 {
    x: i64,
    y: i64,
    dist: i64,
    // Pre-calc
    y_row_dist: i64,
}

impl Sensor1 {
    fn new(x: i64, y: i64, dist: i64, y_row_dist: i64) -> Sensor1 {
        return Sensor1 { x, y, dist, y_row_dist };
    }
}

impl Debug for Sensor1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "({}, {}): {}, {}", self.x, self.y, self.dist, self.y_row_dist);
    }
}

struct Sensor2 {
    x: i64,
    y: i64,
    dist: i64,
    tl_boundary_points: HashSet<(i64, i64), BuildHasherDefault<FnvHasher>>,
    tr_boundary_points: HashSet<(i64, i64), BuildHasherDefault<FnvHasher>>,
    bl_boundary_points: HashSet<(i64, i64), BuildHasherDefault<FnvHasher>>,
    br_boundary_points: HashSet<(i64, i64), BuildHasherDefault<FnvHasher>>
}

impl Sensor2 {
    fn new(x: i64, y: i64, dist: i64) -> Sensor2 {
        return Sensor2 { x, y, dist,
            tl_boundary_points: FnvHashSet::default(),
            tr_boundary_points: FnvHashSet::default(),
            bl_boundary_points: FnvHashSet::default(),
            br_boundary_points: FnvHashSet::default()
        };
    }

    fn get_boundary_points(&mut self) {
        // Top-Left
        let mut x = self.x;
        for y in self.y-self.dist..=self.y {
            self.tl_boundary_points.insert((x, y));
            x -= 1;
        }

        // Top-Right
        x = self.x;
        for y in self.y-self.dist..=self.y {
            self.tr_boundary_points.insert((x, y));
            x += 1;
        }

        // Bottom-Left
        x = self.x-self.dist;
        for y in self.y..=self.y+self.dist {
            self.bl_boundary_points.insert((x, y));
            x += 1;
        }

        // Bottom-Right
        x = self.x+self.dist;
        for y in self.y..=self.y+self.dist {
            self.br_boundary_points.insert((x, y));
            x -= 1;
        }
    }
}

impl Debug for Sensor2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "({}, {}): {}", self.x, self.y, self.dist);
    }
}

impl Problem for Day15 {
    fn part_one(&self, input: &str) -> String {
        let row = 2_000_000;
        let mut min_x = 0;
        let mut max_x = 0;

        let mut count = 0;

        let mut beacons = HashSet::new();

        let binding = input
            .replace(",", "")
            .replace(":", "");

        let mut sensors = binding
            .lines()
            .map(|line| {
                let mut sections = line.split(" ");

                let sx = sections.nth(2).unwrap().split("=").nth(1).unwrap().parse::<i64>().unwrap();
                let sy = sections.nth(0).unwrap().split("=").nth(1).unwrap().parse::<i64>().unwrap();
                let bx = sections.nth(4).unwrap().split("=").nth(1).unwrap().parse::<i64>().unwrap();
                let by = sections.nth(0).unwrap().split("=").nth(1).unwrap().parse::<i64>().unwrap();

                let dist = (sx-bx).abs() + (sy-by).abs();

                min_x = (sx-dist).min(min_x);
                max_x = (sx+dist).max(max_x);

                if row == by { beacons.insert((bx, row)); }

                return Sensor1::new(sx, sy, dist, (sy-row).abs());
            })
            .collect::<Vec<Sensor1>>();

        // We do this to hopefully hit the "break" earlier
        sensors.sort_by(|a, b| a.y_row_dist.partial_cmp(&b.y_row_dist).unwrap());

        // for x in min_x..=max_x {
        //     if beacons.contains(&(x, row)) { continue; }

        //     for s in sensors.iter() {
        //         let temp_dist = (s.x-x).abs() + s.y_row_dist;
        //         if temp_dist > s.dist {
        //             continue;
        //         }
        //         count += 1;
        //         break;
        //     }
        // }

        return format!("{}", count);
    }

    fn part_two(&self, input: &str) -> String {
        let max = 4_000_000 as i64;

        let mut beacons = HashSet::new();

        let binding = input
            .replace(",", "")
            .replace(":", "");

        let sensors = binding
            .lines()
            .enumerate()
            .map(|(i, line)| {
                let mut sections = line.split(" ");

                let sx = sections.nth(2).unwrap().split("=").nth(1).unwrap().parse::<i64>().unwrap();
                let sy = sections.nth(0).unwrap().split("=").nth(1).unwrap().parse::<i64>().unwrap();
                let bx = sections.nth(4).unwrap().split("=").nth(1).unwrap().parse::<i64>().unwrap();
                let by = sections.nth(0).unwrap().split("=").nth(1).unwrap().parse::<i64>().unwrap();

                let dist = (sx-bx).abs() + (sy-by).abs();

                beacons.insert((bx, by));

                let mut s = Sensor2::new(sx, sy, dist);
                s.get_boundary_points();
                println!("Sensor {} done", i);

                return s;
            }).collect::<Vec<Sensor2>>();

        println!("collected sensors");

        let mut intersections = Vec::<(i64, i64)>::new();

        for (i, s1) in sensors.iter().enumerate() {
            println!("Done: {}", i);
            for (j, s2) in sensors.iter().enumerate() {
                if i == j { continue; }

                let i1 = s1.tl_boundary_points.intersection(&s2.tr_boundary_points);
                let i2 = s1.tl_boundary_points.intersection(&s2.br_boundary_points);
                let i3 = s1.bl_boundary_points.intersection(&s2.tr_boundary_points);
                let i4 = s1.bl_boundary_points.intersection(&s2.br_boundary_points);
                for i in i1.chain(i2).chain(i3).chain(i4) {
                    intersections.push(*i);
                }
            }
        }

        println!("intersections got");

        let mut potential = Vec::<(i64, i64)>::new();

        for (x, y) in intersections {
            if y < max {
                potential.push((x, y+1));
            }
            if y > 0 {
                potential.push((x, y-1));
            }
            if x < max {
                potential.push((x+1, y));
            }
            if x > 0 {
                potential.push((x-1, y));
            }
        }

        println!("potentials got");
        println!("point count: {}", potential.len());

        for point in potential.iter() {
            if beacons.contains(&point) { continue; }
            let mut seen = false;

            for s in sensors.iter() {
                let temp_dist = (s.x-point.0).abs() + (s.y-point.1).abs();
                if temp_dist > s.dist {
                    continue;
                }
                seen = true;
                break;
            }

            if !seen { return format!("{:?}", (point.0 * max) + point.1); }
        }


        return format!("Failed to find beacon");
    }
}
