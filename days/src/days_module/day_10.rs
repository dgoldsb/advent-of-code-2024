use crate::days_module::day::Day;
use helpers::grid::grid::Grid;
use helpers::grid::grid_index::GridIndex;
use std::collections::HashSet;
use std::str::FromStr;

pub struct Day10 {}

impl Day for Day10 {
    fn get_id(&self) -> String {
        "day_10".to_string()
    }

    fn get_index(&self) -> u8 {
        10
    }
    fn part_a(&self, input: &String) -> String {
        let grid = Grid::from_str(input).unwrap();
        let trailheads = grid
            .iter()
            .filter(|c| c.value == '0')
            .map(|c| c.index.clone())
            .collect::<Vec<GridIndex>>();
        let mut count = 0;

        for trailhead in trailheads {
            let mut stack = vec![trailhead];
            let mut found_set = HashSet::new();

            while !stack.is_empty() {
                let index = stack.pop().unwrap();
                for neighbour in index.neumann_neighborhood() {
                    let neighbour_cell_option = grid.get_cell(&neighbour);

                    if neighbour_cell_option.is_none() {
                        continue;
                    }

                    if grid.get_cell(&index).unwrap().value.to_digit(10).unwrap() + 1
                        == neighbour_cell_option.unwrap().value.to_digit(10).unwrap()
                    {
                        stack.push(neighbour);
                    }

                    if grid.get_cell(&index).unwrap().value == '9' {
                        found_set.insert(index);
                    }
                }
            }

            count += found_set.len();
        }

        count.to_string()
    }

    fn part_b(&self, input: &String) -> String {
        let grid = Grid::from_str(input).unwrap();
        let trailheads = grid
            .iter()
            .filter(|c| c.value == '0')
            .map(|c| c.index.clone())
            .collect::<Vec<GridIndex>>();
        let mut count = 0;

        for trailhead in trailheads {
            let mut stack = vec![trailhead];

            while !stack.is_empty() {
                let index = stack.pop().unwrap();

                if grid.get_cell(&index).unwrap().value == '9' {
                    count += 1;
                }

                for neighbour in index.neumann_neighborhood() {
                    let neighbour_cell_option = grid.get_cell(&neighbour);

                    if neighbour_cell_option.is_none() {
                        continue;
                    }

                    if grid.get_cell(&index).unwrap().value.to_digit(10).unwrap() + 1
                        == neighbour_cell_option.unwrap().value.to_digit(10).unwrap()
                    {
                        stack.push(neighbour);
                    }
                }
            }
        }

        count.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_a() -> Result<(), String> {
        Day10 {}.test_day_part(&'a')
    }

    #[test]
    fn test_day_b() -> Result<(), String> {
        Day10 {}.test_day_part(&'b')
    }
}
