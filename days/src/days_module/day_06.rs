use crate::days_module::day::Day;
use helpers::grid::grid::Grid;
use helpers::grid::grid_index::GridIndex;
use rayon::prelude::*;
use std::collections::HashSet;
use std::str::FromStr;

pub struct Day06 {}

fn exit_stage<'a>(
    grid: &'a Grid,
    override_index: Option<&'a GridIndex>,
) -> Option<HashSet<(&'a GridIndex, usize)>> {
    let deltas = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut delta_index = 0;
    let mut guard = grid.find_index(&'^').unwrap();
    let mut visited_states = HashSet::new();
    visited_states.insert((guard, delta_index));

    loop {
        let target_cell = grid.get_cell(&GridIndex {
            x: guard.x + deltas.get(delta_index).unwrap().0,
            y: guard.y + deltas.get(delta_index).unwrap().1,
        });

        match target_cell {
            Some(cell) => match cell.value {
                '.' => {
                    if override_index.is_none() || cell.index != *override_index.unwrap() {
                        guard = &cell.index;
                    } else {
                        delta_index = (delta_index + 1) % deltas.len();
                    }
                }
                '#' => delta_index = (delta_index + 1) % deltas.len(),
                '^' => guard = &cell.index,
                _ => break,
            },
            None => break,
        }

        if visited_states.contains(&(guard, delta_index)) {
            return None;
        }

        visited_states.insert((guard, delta_index));
    }
    Some(visited_states)
}

impl Day for Day06 {
    fn get_id(&self) -> String {
        "day_06".to_string()
    }

    fn get_index(&self) -> u8 {
        6
    }

    fn part_a(&self, input: &String) -> String {
        exit_stage(&Grid::from_str(input).unwrap(), None)
            .unwrap()
            .iter()
            .map(|t| t.0)
            .collect::<HashSet<&GridIndex>>()
            .len()
            .to_string()
    }

    fn part_b(&self, input: &String) -> String {
        let grid = Grid::from_str(input).unwrap();
        exit_stage(&grid, None)
            .unwrap()
            .iter()
            .map(|t| t.0)
            .collect::<HashSet<&GridIndex>>()
            .par_iter()
            .map(|i| exit_stage(&grid, Some(i)))
            .filter(|o| o.is_none())
            .count()
            .to_string()
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
