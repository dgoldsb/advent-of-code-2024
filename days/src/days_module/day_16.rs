use crate::days_module::day::Day;
use helpers::grid::grid::Grid;
use helpers::grid::grid_index::{Direction, GridIndex};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::str::FromStr;

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
struct State<'a> {
    index: &'a GridIndex,
    direction: Direction,
}

// State is location + direction, extra cost of 1000 for direction change.
fn dijkstra(grid: &Grid, start_index: &GridIndex, end_index: &GridIndex) -> usize {
    let directions = vec![
        Direction::DOWN,
        Direction::UP,
        Direction::LEFT,
        Direction::RIGHT,
    ];

    let mut g_score: HashMap<State, usize> = HashMap::new();
    let mut came_from: HashMap<State, State> = HashMap::new();
    let mut open_set: HashSet<State> = HashSet::new();
    let mut min_heap: BinaryHeap<(usize, State)> = BinaryHeap::new();

    let state = State {
        index: start_index,
        direction: Direction::RIGHT,
    };
    min_heap.push((usize::MAX, state));
    g_score.insert(state, 0);
    open_set.insert(state);

    while !min_heap.is_empty() {
        let (_, current_state) = min_heap.pop().unwrap();
        open_set.remove(&current_state);

        if current_state.index == end_index {
            continue;
        }

        for direction in &directions {
            let neighbor_cell = grid
                .move_from_cell(current_state.index, &direction)
                .unwrap();

            if neighbor_cell.value == '#' {
                continue;
            }

            let neighbor = State {
                index: &neighbor_cell.index,
                direction: *direction,
            };

            // Calculate the enter cost.
            let mut enter_cost = 1;
            if *direction != current_state.direction {
                enter_cost += 1000;
            }

            // Calculate the tentative G score.
            let predecessor = g_score.get(&current_state).unwrap();
            let tentative_g_score = *predecessor + enter_cost;

            // Continue if we have found a fast route here.
            if tentative_g_score > *g_score.get(&neighbor).unwrap_or(&usize::MAX) {
                continue;
            }

            // Update maps.
            came_from.insert(neighbor, current_state);
            g_score.insert(neighbor, tentative_g_score);

            if !open_set.contains(&neighbor) {
                open_set.insert(neighbor);
                min_heap.push((usize::MAX - tentative_g_score, neighbor));
            }
        }
    }

    g_score
        .iter()
        .filter(|(k, _)| k.index == end_index)
        .map(|(_, v)| v)
        .min()
        .unwrap()
        .clone()
}

pub struct Day16 {}

impl Day for Day16 {
    fn get_id(&self) -> String {
        "day_16".to_string()
    }

    fn get_index(&self) -> u8 {
        16
    }
    fn part_a(&self, input: &String) -> String {
        let grid = Grid::from_str(input).unwrap();
        let start_index = grid.find_index(&'S').unwrap();
        let end_index = grid.find_index(&'E').unwrap();
        dijkstra(&grid, start_index, end_index).to_string()
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
        Day16 {}.test_day_part(&'a')
    }

    #[test]
    fn test_day_b() -> Result<(), String> {
        Day16 {}.test_day_part(&'b')
    }
}
