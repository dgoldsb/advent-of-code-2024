use crate::days_module::day::Day;
use helpers::grid::cell::Cell;
use helpers::grid::grid_index::{Direction, GridIndex};
use helpers::grid::mutable_grid::MutableGrid;
use std::cell::RefCell;
use std::rc::Rc;
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

fn morph_input(input: &str) -> String {
    let mut output = String::new();

    for c in input.chars() {
        match c {
            '.' => {
                output.push('.');
                output.push('.');
            }
            '#' => {
                output.push('#');
                output.push('#');
            }
            'O' => {
                output.push('[');
                output.push(']');
            }
            '@' => {
                output.push('@');
                output.push('.');
            }
            c => output.push(c),
        }
    }

    output
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

fn check_can_push_cell(grid: &MutableGrid, direction: &Direction, index: &GridIndex) -> bool {
    let target = grid.move_from_cell(index, direction);

    if target.is_none() {
        return false;
    }

    let target_rc = target.unwrap();

    let target = target_rc.borrow();
    match target.value {
        '.' => true,
        '[' => {
            if *direction == Direction::DOWN || *direction == Direction::UP {
                let binding = grid
                    .move_from_cell(&target.index, &Direction::RIGHT)
                    .unwrap();
                let matching_target = binding.borrow();
                check_can_push_cell(grid, direction, &target.index)
                    && check_can_push_cell(grid, direction, &matching_target.index)
            } else {
                check_can_push_cell(grid, direction, &target.index)
            }
        }
        ']' => {
            if *direction == Direction::DOWN || *direction == Direction::UP {
                let binding = grid
                    .move_from_cell(&target.index, &Direction::LEFT)
                    .unwrap();
                let matching_target = binding.borrow();
                check_can_push_cell(grid, direction, &target.index)
                    && check_can_push_cell(grid, direction, &matching_target.index)
            } else {
                check_can_push_cell(grid, direction, &target.index)
            }
        }
        '#' => false,
        c => panic!("Unexpected character `{}`", c),
    }
}

fn try_update(rf_cell: Rc<RefCell<Cell>>, value: char) {
    let result = rf_cell.try_borrow_mut();
    if result.is_ok() {
        result.unwrap().value = value;
    }
}

fn execute_push_cell(
    grid: &mut MutableGrid,
    direction: &Direction,
    index: &GridIndex,
) -> GridIndex {
    let target_rc = grid.move_from_cell(index, direction).unwrap();

    // Execute the recursive call.
    match target_rc.borrow().value {
        '.' => {}
        '[' => {
            execute_push_cell(grid, direction, &target_rc.borrow().index);
            if *direction == Direction::DOWN || *direction == Direction::UP {
                let binding = grid
                    .move_from_cell(&target_rc.borrow().index, &Direction::RIGHT)
                    .unwrap();
                let matching_target = binding.borrow();
                execute_push_cell(grid, direction, &matching_target.index);
            }
        }
        ']' => {
            execute_push_cell(grid, direction, &target_rc.borrow().index);
            if *direction == Direction::DOWN || *direction == Direction::UP {
                let binding = grid
                    .move_from_cell(&target_rc.borrow().index, &Direction::LEFT)
                    .unwrap();
                let matching_target = binding.borrow();
                execute_push_cell(grid, direction, &matching_target.index);
            }
        }
        c => panic!("Unexpected character `{}`", c),
    }

    // Overwrite the target.
    let mut target_cell_mut = target_rc.borrow_mut();
    match target_cell_mut.value {
        '.' => {
            target_cell_mut.value = grid.get_cell(index).unwrap().borrow().value;
        }
        '[' => {
            target_cell_mut.value = '.';
            if *direction == Direction::DOWN || *direction == Direction::UP {
                let binding = grid
                    .move_from_cell(&target_cell_mut.index, &Direction::RIGHT)
                    .unwrap();
                try_update(binding, '.');
            }
        }
        ']' => {
            target_cell_mut.value = '.';
            if *direction == Direction::DOWN || *direction == Direction::UP {
                let binding = grid
                    .move_from_cell(&target_cell_mut.index, &Direction::LEFT)
                    .unwrap();
                try_update(binding, '.');
            }
        }
        c => panic!("Unexpected character `{}`", c),
    }

    target_cell_mut.index
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
        let morphed_input = morph_input(input);
        let (mut grid, directions) = parse_input(&morphed_input);

        let mut robot_index = grid.find_index(&'@').unwrap().clone();
        for direction in directions {
            grid.print();
            if check_can_push_cell(&grid, &direction, &robot_index) {
                let new_robot_index = execute_push_cell(&mut grid, &direction, &robot_index);
                let binding = grid.get_cell(&robot_index).unwrap();
                let mut current_cell_mut = binding.borrow_mut();
                current_cell_mut.value = '.';
                robot_index = new_robot_index;
            }
        }

        grid.iter()
            .map(|cell| cell.borrow().clone())
            .filter(|cell| cell.value == '[')
            .map(|cell| cell.index.y + cell.index.x * 100)
            .sum::<isize>()
            .to_string()
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
