use std::{collections::HashMap, str::FromStr};

use helpers::{grid::grid::Grid, manhattan_distance};

use crate::days_module::day::Day;

/* It is a single track, so we can build a table of steps with DFS.
 * Each cheat must start from the original track, and end on the original track, and have a Manhattan distance of two.
 * The time save is the difference in the table between the start and end.
*/

fn solve(input: &String, duration: usize) -> usize {
    let grid = Grid::from_str(&input).unwrap();

    // Build the visited map.
    let start = grid.find_index(&'S').unwrap();
    let mut visited = HashMap::new();
    let mut stack = vec![start.clone()];
    visited.insert(start.clone(), 0);

    while let Some(current) = stack.pop() {
        let cost = *visited.get(&current).unwrap();
        for neighbor in current.neumann_neighborhood() {
            if visited.contains_key(&neighbor)
                || grid.get_cell(&neighbor).is_none()
                || grid.get_cell(&neighbor).unwrap().value == '#'
            {
                continue;
            }
            visited.insert(neighbor, cost + 1);
            stack.push(neighbor);
        }
    }

    // Iterate over possible shortcuts.
    let mut at_least_100_count = 0;
    for s in visited.keys() {
        for e in visited.keys() {
            let manhattan_distance =
                manhattan_distance(&(s.x as usize, s.y as usize), &(e.x as usize, e.y as usize));
            if manhattan_distance <= duration {
                // Evaluate time save.
                let ps_saved = visited.get(e).unwrap()
                    - visited.get(s).unwrap()
                    - (manhattan_distance as isize);
                if ps_saved >= 100 {
                    at_least_100_count += 1;
                }
            }
        }
    }

    at_least_100_count
}

pub struct Day20 {}

impl Day for Day20 {
    fn get_id(&self) -> String {
        "day_20".to_string()
    }

    fn get_index(&self) -> u8 {
        20
    }
    fn part_a(&self, input: &String) -> String {
        solve(&input, 2).to_string()
    }

    fn part_b(&self, input: &String) -> String {
        solve(&input, 20).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_a() -> Result<(), String> {
        Day20 {}.test_day_part(&'a')
    }

    #[test]
    fn test_day_b() -> Result<(), String> {
        Day20 {}.test_day_part(&'b')
    }
}
