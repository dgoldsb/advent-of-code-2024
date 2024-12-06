use crate::days_module::day::Day;
use helpers::grid::grid::Grid;
use helpers::grid::grid_index::GridIndex;
use rayon::prelude::*;
use std::collections::HashSet;
use std::str::FromStr;

pub struct FastSet<'a> {
    dimensions: &'a (isize, isize),
    index_lookup: Vec<&'a GridIndex>,
    memory: Vec<bool>,
}

impl<'a> FastSet<'_> {
    pub fn new(grid: &'a Grid) -> FastSet {
        let index_lookup = grid.iter().map(|g| &g.index).collect::<Vec<&GridIndex>>();
        let size = index_lookup.len() * 4;
        FastSet {
            dimensions: &grid.dimensions,
            index_lookup,
            memory: vec![false; size],
        }
    }

    pub fn insert(&mut self, key: (&GridIndex, isize)) {
        let index = self.convert_to_memory_index(key);
        self.memory[index] = true;
    }

    pub fn contains(&self, key: &(&GridIndex, isize)) -> bool {
        self.memory[self.convert_to_memory_index(*key)]
    }

    fn convert_to_memory_index(&self, key: (&GridIndex, isize)) -> usize {
        let memory_index =
            key.0.x * self.dimensions.1 + key.0.y + key.1 * (self.dimensions.0 * self.dimensions.1);
        memory_index as usize
    }

    pub fn recover_grid_index(&self, index: &usize) -> &GridIndex {
        let mod_index = index % self.index_lookup.len();
        &self.index_lookup[mod_index]
    }
}

pub struct Day06 {}

fn exit_stage<'a>(grid: &'a Grid, override_index: Option<&'a GridIndex>) -> Option<FastSet<'a>> {
    let deltas = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut delta_index = 0;
    let mut guard = grid.find_index(&'^').unwrap();
    let mut visited_states = FastSet::new(grid);
    visited_states.insert((guard, delta_index));

    loop {
        let target_cell = grid.get_cell(&GridIndex {
            x: guard.x + deltas.get(delta_index as usize).unwrap().0,
            y: guard.y + deltas.get(delta_index as usize).unwrap().1,
        });

        match target_cell {
            Some(cell) => match cell.value {
                '.' => {
                    if override_index.is_none() || cell.index != *override_index.unwrap() {
                        guard = &cell.index;
                    } else {
                        delta_index = (delta_index + 1) % deltas.len() as isize;
                    }
                }
                '#' => delta_index = (delta_index + 1) % deltas.len() as isize,
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
        let grid = Grid::from_str(input).unwrap();
        let visited_states = exit_stage(&grid, None).unwrap();

        visited_states
            .memory
            .iter()
            .enumerate()
            .filter(|(_, b)| **b)
            .map(|(i, _)| visited_states.recover_grid_index(&i))
            .collect::<HashSet<&GridIndex>>()
            .len()
            .to_string()
    }

    fn part_b(&self, input: &String) -> String {
        let grid = Grid::from_str(input).unwrap();
        let visited_states = exit_stage(&grid, None).unwrap();
        let distinct_visited_grid_indexes = visited_states
            .memory
            .iter()
            .enumerate()
            .filter(|&(_, &b)| b)
            .map(|(i, _)| visited_states.recover_grid_index(&i))
            .collect::<HashSet<&GridIndex>>();

        distinct_visited_grid_indexes
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
