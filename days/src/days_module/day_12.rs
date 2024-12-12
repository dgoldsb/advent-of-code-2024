use crate::days_module::day::Day;
use helpers::grid::grid::Grid;
use helpers::grid::grid_index::GridIndex;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

pub struct Day12 {}

impl Day for Day12 {
    fn get_id(&self) -> String {
        "day_12".to_string()
    }

    fn get_index(&self) -> u8 {
        12
    }

    fn part_a(&self, input: &String) -> String {
        // Find regions.
        let grid = Grid::from_str(input).unwrap();
        let mut seen_set = HashSet::new();
        let mut regions: HashMap<GridIndex, HashSet<GridIndex>> = HashMap::new();

        for cell in grid.iter() {
            if seen_set.contains(&cell.index) {
                continue;
            }

            let mut region = HashSet::new();

            let mut stack = vec![cell.index];
            while let Some(i) = stack.pop() {
                if seen_set.contains(&i) {
                    continue;
                }

                if grid.get_cell(&i).is_none() || grid.get_cell(&i).unwrap().value != cell.value {
                    continue;
                }

                seen_set.insert(i);
                region.insert(i);

                for new_index in i.neumann_neighborhood() {
                    stack.push(new_index);
                }
            }

            regions.insert(cell.index, region);
        }

        // Score regions.
        let mut result = 0;

        for region in regions.values() {
            let mut perimeter = 0;

            for index in region {
                for neighbor in index.neumann_neighborhood() {
                    if !region.contains(&neighbor) {
                        perimeter += 1;
                    }
                }
            }

            result += perimeter * region.len();
        }

        println!("{}", result);

        "".to_string()
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
        Day12 {}.test_day_part(&'a')
    }

    #[test]
    fn test_day_b() -> Result<(), String> {
        Day12 {}.test_day_part(&'b')
    }
}
