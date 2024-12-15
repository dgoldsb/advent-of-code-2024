use crate::days_module::day::Day;
use helpers::grid::grid_index::{Direction, GridIndex};
use helpers::grid::mutable_grid::MutableGrid;
use std::str::FromStr;

pub struct Day15 {}

fn parse_input(input: &str) -> (MutableGrid, Vec<Direction>) {
    let mut iterator = input.split("\n\n");
    let grid = MutableGrid::from_str(iterator.next().unwrap()).unwrap();
    let directions = iterator
        .next()
        .unwrap()
        .chars()
        .filter(|c| *c != '\n')
        .map({
            |c| match c {
                '^' => Direction::UP,
                '>' => Direction::RIGHT,
                '<' => Direction::LEFT,
                'v' => Direction::DOWN,
                c => panic!("Unsupported character `{}`", c),
            }
        })
        .collect::<Vec<_>>();
    (grid, directions)
}

fn push_cell(
    grid: &mut MutableGrid,
    direction: &Direction,
    index: &GridIndex,
) -> Result<GridIndex, ()> {
    let target = grid.move_from_cell(index, direction);

    if target.is_none() {
        return Err(());
    }

    let target_rc = target.unwrap();
    match target_rc.borrow().value {
        '.' => {}
        'O' => {
            let result = push_cell(grid, direction, &target_rc.borrow().index);
            if result.is_err() {
                return Err(());
            }
        }
        '#' => {
            return Err(());
        }
        c => panic!("Unexpected character `{}`", c),
    }

    let mut target_cell_mut = target_rc.borrow_mut();
    target_cell_mut.value = grid.get_cell(index).unwrap().borrow().value;
    Ok(target_cell_mut.index)
}

impl Day for Day15 {
    fn get_id(&self) -> String {
        "day_15".to_string()
    }

    fn get_index(&self) -> u8 {
        15
    }
    fn part_a(&self, input: &String) -> String {
        let (mut grid, directions) = parse_input(input);

        let mut robot_index = grid.find_index(&'@').unwrap().clone();
        for direction in directions {
            let result = push_cell(&mut grid, &direction, &robot_index);

            if result.is_ok() {
                let binding = grid.get_cell(&robot_index).unwrap();
                let mut current_cell_mut = binding.borrow_mut();
                current_cell_mut.value = '.';
                robot_index = result.unwrap();
            }
        }

        grid.iter()
            .map(|cell| cell.borrow().clone())
            .filter(|cell| cell.value == 'O')
            .map(|cell| cell.index.y + cell.index.x * 100)
            .sum::<isize>()
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
        Day15 {}.test_day_part(&'a')
    }

    #[test]
    fn test_day_b() -> Result<(), String> {
        Day15 {}.test_day_part(&'b')
    }
}
