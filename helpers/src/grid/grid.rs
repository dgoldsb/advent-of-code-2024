use crate::grid::cell::Cell;
use crate::grid::grid_index::{Direction, GridIndex};
use core::slice::Iter;
use std::str::FromStr;

pub struct Grid {
    cells: Vec<Cell>,
    pub dimensions: (isize, isize),
}

impl Grid {
    pub fn iter(&self) -> Iter<Cell> {
        self.cells.iter()
    }

    pub fn find_index(&self, value: &char) -> Result<&GridIndex, ()> {
        for cell in &self.cells {
            if cell.value == *value {
                return Ok(&cell.index);
            }
        }
        return Err(());
    }

    pub fn get_cell(&self, index: &GridIndex) -> Option<&Cell> {
        self.get_cell_by_index(&(index.x, index.y))
    }

    pub fn get_cell_by_index(&self, index: &(isize, isize)) -> Option<&Cell> {
        // Return None if the index is not in the grid.
        if index.0 < 0
            || index.0 >= self.dimensions.0
            || index.1 < 0
            || index.1 >= self.dimensions.1
        {
            return None;
        }

        let vec_index: usize = (index.0 * self.dimensions.1 + index.1).try_into().unwrap();
        return Some(&self.cells[vec_index]);
    }

    pub fn print(&self) {
        let mut last_y = 0;
        for cell in &self.cells {
            let new_y = cell.index.y;
            if new_y < last_y {
                print!("\n");
            }
            last_y = new_y;
            print!("{}", cell.value.to_string());
        }
        print!("\n");
    }

    pub fn move_from_cell(&self, index: &GridIndex, direction: &Direction) -> Option<&Cell> {
        match direction {
            Direction::UP => self.get_cell(&GridIndex {
                x: index.x - 1,
                y: index.y,
            }),
            Direction::DOWN => self.get_cell(&GridIndex {
                x: index.x + 1,
                y: index.y,
            }),
            Direction::RIGHT => self.get_cell(&GridIndex {
                x: index.x,
                y: index.y + 1,
            }),
            Direction::LEFT => self.get_cell(&GridIndex {
                x: index.x,
                y: index.y - 1,
            }),
        }
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(input: &str) -> Result<Grid, Self::Err> {
        let mut cells = Vec::new();

        let mut max_column = 0;
        let mut row_index = 0;
        for line in input.split("\n") {
            let mut column_index = 0;
            for char_ in line.chars() {
                cells.push(Cell {
                    index: GridIndex {
                        x: row_index,
                        y: column_index,
                    },
                    value: char_,
                });
                column_index += 1;
            }
            max_column = column_index;
            row_index += 1;
        }

        let dimensions = (row_index, max_column);
        return Ok(Grid { cells, dimensions });
    }
}
