use crate::problem::Problem;

pub struct Day9 {}

#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32
}

impl Problem for Day9 {
    fn part_one(&self, input: &str) -> String {
        // Shouldn't get bigger than this, no need to count potential max tbh
        let mut truth = vec![vec![0; 1000]; 1000];

        // Starting in centre to allow for any degree of movement
        let mut h = Point { x: 500, y: 500 };
        let mut t = Point { x: 500, y: 500 };

        // Problem grid starts bottom left so here U = down and D = up
        for cmd in input.lines() {
            let mut split = cmd.split(" ");
            let dir = split.next().unwrap();
            let mut count = split.next().unwrap().parse::<usize>().unwrap();

            while count > 0 {
                match dir {
                    "U" => h.y += 1,
                    "D" => h.y -= 1,
                    "L" => h.x -= 1,
                    "R" => h.x += 1,
                    _ => println!("Cmd not found"),
                }

                // Check coord diff
                let x_diff = (h.x - t.x).abs();
                let y_diff = (h.y - t.y).abs();

                // Check if tail has been offset enough to need to catch up
                if x_diff > 1 {
                    // Offset by x:2
                    if h.x > t.x { t.x += 1 } else { t.x -= 1 }
                    if y_diff > 0 {
                        // Offset by y:1
                        if h.y > t.y { t.y += 1 } else { t.y -= 1 }
                    }
                } else if y_diff > 1 {
                    // Offset by y:2
                    if h.y > t.y { t.y += 1 } else { t.y -= 1 }
                    if x_diff > 0 {
                        // Offset by x:1
                        if h.x > t.x { t.x += 1 } else { t.x -= 1 }
                    }
                }

                truth[t.y as usize][t.x as usize] = 1;
                count -= 1;
            }
        }

        return format!("{:?}", truth.iter().map(|row| row.iter().sum::<usize>()).sum::<usize>());
    }

    fn part_two(&self, input: &str) -> String {
        // Shouldn't get bigger than this, no need to count potential max tbh
        let mut truth = vec![vec![0; 1000]; 1000];

        // Starting in centre to allow for any degree of movement
        let mut snake = vec![Point { x: 500, y: 500 }; 10];

        // Problem grid starts bottom left so here U = down and D = up
        for cmd in input.lines() {
            let mut split = cmd.split(" ");
            let dir = split.next().unwrap();
            let mut count = split.next().unwrap().parse::<usize>().unwrap();

            while count > 0 {
                match dir {
                    "U" => snake[0].y += 1,
                    "D" => snake[0].y -= 1,
                    "L" => snake[0].x -= 1,
                    "R" => snake[0].x += 1,
                    _ => println!("Cmd not found"),
                }

                // Compare current segment with previous one
                // More generic version of part 1
                for i in 1..snake.len() {
                    // Check coord diff
                    let x_diff = (snake[i-1].x - snake[i].x).abs();
                    let y_diff = (snake[i-1].y - snake[i].y).abs();

                    // Check if tail has been offset enough to need to catch up
                    if x_diff > 1 {
                        // Offset by x:2
                        if snake[i-1].x > snake[i].x { snake[i].x += 1 } else { snake[i].x -= 1 }
                        if y_diff > 0 {
                            // Offset by y:1
                            if snake[i-1].y > snake[i].y { snake[i].y += 1 } else { snake[i].y -= 1 }
                        }
                    } else if y_diff > 1 {
                        // Offset by y:2
                        if snake[i-1].y > snake[i].y { snake[i].y += 1 } else { snake[i].y -= 1 }
                        if x_diff > 0 {
                            // Offset by x:1
                            if snake[i-1].x > snake[i].x { snake[i].x += 1 } else { snake[i].x -= 1 }
                        }
                    } else {
                        // The item hasn't moved so none of it's segments afterwards have
                        // So we don't need to check them
                        break;
                    }
                }

                truth[snake[snake.len()-1].y as usize][snake[snake.len()-1].x as usize] = 1;
                count -= 1;
            }
        }

        return format!("{:?}", truth.iter().map(|row| row.iter().sum::<usize>()).sum::<usize>());
    }
}
