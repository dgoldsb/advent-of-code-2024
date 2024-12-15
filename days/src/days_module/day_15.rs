use crate::days_module::day::Day;
use helpers::grid::mutable_grid::{Direction, Grid};
use std::str::FromStr;
use helpers::grid::grid_index::GridIndex;

pub struct Day15 {}

fn parse_input(input: &str) -> (Grid, Vec<Direction>) {
    let mut iterator = input.split("\n\n");
    let grid = Grid::from_str(iterator.next().unwrap()).unwrap();
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

fn push_cell(grid: &mut Grid, direction: &Direction, index: &GridIndex) -> Result<GridIndex, ()> {
    let target = grid.move_from_cell(index, direction);

    if target.is_none() {
        return Err(());
    }

    let current_cell_rc = grid.get_cell(index).unwrap();
    let current_cell_value = current_cell_rc.borrow().value;
    let target_cell_rc = target.unwrap();
    let target_cell = target_cell_rc.borrow();
    match target_cell.value {
        '.' => {
            current_cell_rc.borrow_mut().value = '.';
            target_cell_rc.borrow_mut().value = current_cell_value;
            Ok(target_cell.index)
        },
        'O' => {
            let result = push_cell(grid, direction, &target_cell.index);
            if result.is_err() {
                Err(())
            } else {
                current_cell_rc.borrow_mut().value = '.';
                target_cell_rc.borrow_mut().value = current_cell_value;
                Ok(target_cell.index)
            }
        }
        '#' => { Err(()) },
        c => panic!("Unexpected character `{}`", c),
    }

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
                robot_index = result.unwrap();
            }
        }
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
        Day15 {}.test_day_part(&'a')
    }

    #[test]
    fn test_day_b() -> Result<(), String> {
        Day15 {}.test_day_part(&'b')
    }
}
