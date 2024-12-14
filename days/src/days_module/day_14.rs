use crate::days_module::day::Day;
use helpers::{find_numbers, print_sparse_grid};
use std::collections::HashSet;

pub struct Day14 {}

struct Robot {
    position: (isize, isize),
    velocity: (isize, isize),
}

fn parse_input(input: &str) -> Vec<Robot> {
    let mut numbers = find_numbers(input);
    numbers.reverse();
    let mut robots = Vec::new();

    while !numbers.is_empty() {
        let y = numbers.pop().unwrap();
        let x = numbers.pop().unwrap();
        let vy = numbers.pop().unwrap();
        let vx = numbers.pop().unwrap();
        let robot = Robot {
            position: (x, y),
            velocity: (vx, vy),
        };
        robots.push(robot);
    }

    robots
}

fn is_christmas_tree(robots: &Vec<Robot>, grid_size: &(isize, isize)) -> bool {
    let position_set = robots
        .iter()
        .map(|r| (r.position.0 as usize, r.position.1 as usize))
        .collect::<HashSet<_>>();
    let output = print_sparse_grid(&position_set, (grid_size.0 as usize, grid_size.1 as usize));
    output.contains("###############")
}

impl Day for Day14 {
    fn get_id(&self) -> String {
        "day_14".to_string()
    }

    fn get_index(&self) -> u8 {
        14
    }
    fn part_a(&self, input: &String) -> String {
        let grid_size = (103, 101);
        let seconds_to_wait = 100;
        let mut robots = parse_input(&input);

        for _ in 0..seconds_to_wait {
            for robot in robots.iter_mut() {
                robot.position = (
                    (robot.position.0 + robot.velocity.0 + grid_size.0) % grid_size.0,
                    (robot.position.1 + robot.velocity.1 + grid_size.1) % grid_size.1,
                );
            }
        }

        let middle = (grid_size.0 / 2, grid_size.1 / 2);
        let mut buckets = vec![0, 0, 0, 0];
        for robot in &robots {
            if robot.position.0 < middle.0 && robot.position.1 < middle.1 {
                buckets[0] += 1;
            } else if robot.position.0 > middle.0 && robot.position.1 < middle.1 {
                buckets[1] += 1;
            } else if robot.position.0 < middle.0 && robot.position.1 > middle.1 {
                buckets[2] += 1;
            } else if robot.position.0 > middle.0 && robot.position.1 > middle.1 {
                buckets[3] += 1;
            }
        }

        (buckets[0] * buckets[1] * buckets[2] * buckets[3]).to_string()
    }

    fn part_b(&self, input: &String) -> String {
        let grid_size = (103, 101);
        let mut robots = parse_input(&input);

        let mut seconds_waited = 0;
        loop {
            for robot in robots.iter_mut() {
                robot.position = (
                    (robot.position.0 + robot.velocity.0 + grid_size.0) % grid_size.0,
                    (robot.position.1 + robot.velocity.1 + grid_size.1) % grid_size.1,
                );
            }
            seconds_waited += 1;

            if is_christmas_tree(&robots, &grid_size) {
                break;
            }
        }

        seconds_waited.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_a() -> Result<(), String> {
        Day14 {}.test_day_part(&'a')
    }

    #[test]
    fn test_day_b() -> Result<(), String> {
        Day14 {}.test_day_part(&'b')
    }
}
