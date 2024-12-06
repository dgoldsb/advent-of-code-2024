use crate::days_module::day::Day;
use helpers::grid::cell::Cell;
use helpers::grid::grid::Grid;
use helpers::grid::grid_index::GridIndex;
use std::collections::HashSet;
use std::str::FromStr;

pub struct Day06 {}

impl Day for Day06 {
    fn get_id(&self) -> String {
        "day_06".to_string()
    }

    fn get_index(&self) -> u8 {
        6
    }
    fn part_a(&self, input: &String) -> String {
        let grid = Grid::from_str(input).unwrap();
        let deltas = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
        let mut delta_index = 0;
        let mut guard = grid.find_index(&'^').unwrap();
        let mut visited = HashSet::new();

        visited.insert((guard.x, guard.y));

        loop {
            let target_cell = grid.get_cell(&GridIndex {
                x: guard.x + deltas.get(delta_index).unwrap().0,
                y: guard.y + deltas.get(delta_index).unwrap().1,
            });

            match target_cell {
                Some(cell) => match cell.value {
                    '.' => guard = &cell.index,
                    '#' => delta_index = (delta_index + 1) % deltas.len(),
                    '^' => guard = &cell.index,
                    _ => break,
                },
                None => break,
            }
            visited.insert((guard.x, guard.y));
        }

        visited.len().to_string()
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
        Day06 {}.test_day_part(&'a')
    }

    #[test]
    fn test_day_b() -> Result<(), String> {
        Day06 {}.test_day_part(&'b')
    }
}
