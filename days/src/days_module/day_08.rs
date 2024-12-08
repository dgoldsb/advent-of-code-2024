use crate::days_module::day::Day;
use helpers::grid::cell::Cell;
use helpers::grid::grid::Grid;
use helpers::grid::grid_index::GridIndex;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

pub struct Day08 {}

impl Day for Day08 {
    fn get_id(&self) -> String {
        "day_08".to_string()
    }

    fn get_index(&self) -> u8 {
        8
    }

    fn part_a(&self, input: &String) -> String {
        // Signals
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

        let mut anti_nodes = HashSet::new();

        for set in antennae.values() {
            for a in set.iter() {
                for b in set.iter() {
                    if a == b {
                        continue;
                    }
                    anti_nodes.insert(GridIndex {
                        x: a.x + (a.x - b.x),
                        y: a.y + (a.y - b.y),
                    });
                    anti_nodes.insert(GridIndex {
                        x: b.x + (b.x - a.x),
                        y: b.y + (b.y - a.y),
                    });
                }
            }
        }

        anti_nodes
            .iter()
            .filter(|i| grid.get_cell(*i).is_some())
            .collect::<Vec<_>>()
            .len()
            .to_string()
    }

    fn part_b(&self, input: &String) -> String {
        "".to_string()
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
