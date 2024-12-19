use crate::days_module::day::Day;
use helpers::grid::grid::Grid;
use helpers::grid::grid_index::{Direction, GridIndex};
use lazy_static::lazy_static;
use std::cell::RefCell;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::str::FromStr;

lazy_static! {
    static ref DIRECTIONS: Vec<Direction> = vec![
        Direction::DOWN,
        Direction::UP,
        Direction::LEFT,
        Direction::RIGHT,
    ];
}

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
struct State<'a> {
    index: &'a GridIndex,
    direction: Direction,
}

// State is location + direction, extra cost of 1000 for direction change.
fn dijkstra<'a>(
    grid: &'a Grid,
    start_index: &'a GridIndex,
    end_index: &'a GridIndex,
) -> (usize, HashMap<State<'a>, usize>) {
    let mut g_score: HashMap<State, usize> = HashMap::new();
    let mut came_from: HashMap<State, State> = HashMap::new();
    let mut open_set: HashSet<State> = HashSet::new();
    let mut max_heap: BinaryHeap<(usize, State)> = BinaryHeap::new();

    let state = State {
        index: start_index,
        direction: Direction::RIGHT,
    };
    max_heap.push((usize::MAX, state));
    g_score.insert(state, 0);
    open_set.insert(state);

    while !max_heap.is_empty() {
        let (current, current_state) = max_heap.pop().unwrap();
        open_set.remove(&current_state);

        if current_state.index == end_index {
            return (usize::MAX - current, g_score);
        }

        for direction in DIRECTIONS.iter() {
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
                max_heap.push((usize::MAX - tentative_g_score, neighbor));
            }
        }
    }
    panic!("No path found!");
}

fn pruned_dfs<'a>(
    grid: &'a Grid,
    current: &'a GridIndex,
    current_direction: &'a Direction,
    score: usize,
    max_score: &'a usize,
    visited: &'a RefCell<HashSet<&'a GridIndex>>,
    g_scores: &'a HashMap<State, usize>,
) -> bool {
    if score > *max_score {
        return false;
    }

    if grid.get_cell(&current).unwrap().value == 'E' {
        visited.borrow_mut().insert(current); // only for
        return true;
    }

    if score
        > *g_scores
            .get(&State {
                index: current,
                direction: *current_direction,
            })
            .unwrap()
    {
        return false;
    }

    let mut any_found = false;

    for direction in DIRECTIONS.iter() {
        match direction {
            Direction::DOWN => {
                if *current_direction == Direction::UP {
                    continue;
                }
            }
            Direction::UP => {
                if *current_direction == Direction::DOWN {
                    continue;
                }
            }
            Direction::LEFT => {
                if *current_direction == Direction::RIGHT {
                    continue;
                }
            }
            Direction::RIGHT => {
                if *current_direction == Direction::LEFT {
                    continue;
                }
            }
        }

        let neighbor_cell = grid.move_from_cell(current, &direction).unwrap();

        if neighbor_cell.value == '#' {
            continue;
        }

        let mut enter_cost = 1;
        if direction != current_direction {
            enter_cost += 1000;
        }

        let result = pruned_dfs(
            grid,
            &neighbor_cell.index,
            direction,
            score + enter_cost,
            max_score,
            visited,
            g_scores,
        );

        any_found = any_found || result;
    }

    if any_found {
        visited.borrow_mut().insert(current);
        true
    } else {
        false
    }
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
        dijkstra(&grid, start_index, end_index).0.to_string()
    }

    fn part_b(&self, input: &String) -> String {
        let grid = Grid::from_str(input).unwrap();
        let start_index = grid.find_index(&'S').unwrap();
        let end_index = grid.find_index(&'E').unwrap();

        let (shortest_path, g_scores) = dijkstra(&grid, start_index, end_index);
        let visited_set = RefCell::new(HashSet::new());

        pruned_dfs(
            &grid,
            start_index,
            &Direction::RIGHT,
            0,
            &shortest_path,
            &visited_set,
            &g_scores,
        );

        let x = visited_set.borrow().len().to_string();
        x
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
