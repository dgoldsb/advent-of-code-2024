use crate::days_module::day::Day;
use helpers::find_numbers;
use helpers::grid::grid_index::{Direction, GridIndex};
use lazy_static::lazy_static;
use std::collections::{BinaryHeap, HashMap, HashSet};


lazy_static! {
    static ref DIRECTIONS: Vec<Direction> = vec![
        Direction::DOWN,
        Direction::UP,
        Direction::LEFT,
        Direction::RIGHT,
    ];
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
struct State {
    index: GridIndex,
    time_spent: isize,
}

// State is location + direction, extra cost of 1000 for direction change.
fn dijkstra(
    start_index: &GridIndex,
    corruptions: &HashMap<isize, HashSet<GridIndex>>,
) -> usize {
    let grid_max = 70;
    let pre_fallen = 1024;

    let end_index = GridIndex { x: grid_max, y: grid_max };

    let mut g_score: HashMap<State, usize> = HashMap::new();
    let mut came_from: HashMap<State, State> = HashMap::new();
    let mut open_set: HashSet<State> = HashSet::new();
    let mut min_heap: BinaryHeap<(usize, State)> = BinaryHeap::new();

    let state = State {
        index: start_index.clone(),
        time_spent: pre_fallen.clone(),
    };
    min_heap.push((usize::MAX, state));
    g_score.insert(state, 0);
    open_set.insert(state);

    while !min_heap.is_empty() {
        let (current, current_state) = min_heap.pop().unwrap();

        open_set.remove(&current_state);

        if current_state.index == end_index {
            return usize::MAX - current;
        }

        for direction in DIRECTIONS.iter() {
            let neighbor = match direction {
                Direction::UP => GridIndex {
                    x: current_state.index.x - 1,
                    y: current_state.index.y,
                },
                Direction::DOWN => GridIndex {
                    x: current_state.index.x + 1,
                    y: current_state.index.y,
                },
                Direction::RIGHT => GridIndex {
                    x: current_state.index.x,
                    y: current_state.index.y + 1,
                },
                Direction::LEFT => GridIndex {
                    x: current_state.index.x,
                    y: current_state.index.y - 1,
                },
            };

            if neighbor.x < 0 || neighbor.y < 0 || neighbor.x > grid_max || neighbor.y > grid_max {
                continue;
            }

            if corruptions.get(&current_state.time_spent).unwrap_or(&HashSet::new()).contains(&neighbor) {
                continue;
            }

            let neighbor = State {
                index: neighbor,
                time_spent: current_state.time_spent,
            };

            // Calculate the tentative G score.
            let predecessor = g_score.get(&current_state).unwrap();
            let tentative_g_score = *predecessor + 1;

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
    panic!("No path found!");
}

pub struct Day18 {}

impl Day for Day18 {
    fn get_id(&self) -> String {
        "day_18".to_string()
    }

    fn get_index(&self) -> u8 {
        18
    }
    fn part_a(&self, input: &String) -> String {
        let binding = find_numbers(&input);
        let mut number_iterator = binding.iter();
        let mut corruptions: HashMap<isize, HashSet<GridIndex>> = HashMap::new();
        let mut index = 1;  // TODO off by one?

        corruptions.insert(0, HashSet::new());

        while let Some(x) = number_iterator.next() {
            let y = number_iterator.next().unwrap();
            let corruption: GridIndex = GridIndex { x: *x, y: *y };
            
            let mut new_set = HashSet::new();
            new_set.insert(corruption);
            new_set.extend(corruptions.get(&(index - 1)).unwrap());
            corruptions.insert(index, new_set);


            index += 1;
        }

        dijkstra(&GridIndex { x: 0, y: 0 }, &corruptions).to_string()
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
        Day18 {}.test_day_part(&'a')
    }

    #[test]
    fn test_day_b() -> Result<(), String> {
        Day18 {}.test_day_part(&'b')
    }
}
