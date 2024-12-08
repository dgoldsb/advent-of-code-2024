use crate::days_module::day::Day;
use helpers::grid::grid::Grid;
use helpers::grid::grid_index::GridIndex;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

pub struct Day08 {}

fn generate_antinodes(a: &GridIndex, b: &GridIndex, grid: &Grid, part_b: bool) -> Vec<GridIndex> {
    let mut found_nodes = Vec::new();

    let mut a_index = a.clone();

    if part_b {
        found_nodes.push(a_index);
    }

    let ax_delta = a.x - b.x;
    let ay_delta = a.y - b.y;
    loop {
        a_index = GridIndex {
            x: a_index.x + ax_delta,
            y: a_index.y + ay_delta,
        };

        if grid.get_cell(&a_index).is_some() {
            found_nodes.push(a_index);
        }

        if grid.get_cell(&a_index).is_none() || !part_b {
            break;
        }
    }

    let mut b_index = b.clone();

    if part_b {
        found_nodes.push(b_index);
    }

    let bx_delta = b.x - a.x;
    let by_delta = b.y - a.y;
    loop {
        b_index = GridIndex {
            x: b_index.x + bx_delta,
            y: b_index.y + by_delta,
        };

        if grid.get_cell(&b_index).is_some() {
            found_nodes.push(b_index);
        }

        if grid.get_cell(&b_index).is_none() || !part_b {
            break;
        }
    }

    found_nodes
}

fn solve(input: &str, part_b: bool) -> usize {
    let grid = Grid::from_str(input).unwrap();
    let mut antennae: HashMap<&char, HashSet<&GridIndex>> = HashMap::new();
    for cell in grid.iter() {
        if cell.value == '.' {
            continue;
        }

        match antennae.get_mut(&cell.value) {
            Some(set) => {
                set.insert(&cell.index);
            }
            None => {
                let mut set = HashSet::new();
                set.insert(&cell.index);
                antennae.insert(&cell.value, set);
            }
        }
    }

    let mut antinodes = HashSet::new();
    antennae.values().for_each(|set| {
        for a in set {
            for b in set {
                if a == b {
                    continue;
                }

                antinodes.extend(generate_antinodes(a, b, &grid, part_b));
            }
        }
    });

    antinodes.len()
}

impl Day for Day08 {
    fn get_id(&self) -> String {
        "day_08".to_string()
    }

    fn get_index(&self) -> u8 {
        8
    }

    fn part_a(&self, input: &String) -> String {
        solve(input, false).to_string()
    }

    fn part_b(&self, input: &String) -> String {
        solve(input, true).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_a() -> Result<(), String> {
        Day08 {}.test_day_part(&'a')
    }

    #[test]
    fn test_day_b() -> Result<(), String> {
        Day08 {}.test_day_part(&'b')
    }
}
