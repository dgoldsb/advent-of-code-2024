use crate::grid::cell::Cell;
use crate::grid::grid_index::GridIndex;
use std::cell::RefCell;
use std::rc::Rc;
use std::str::FromStr;

pub struct MutableGrid {
    cells: Vec<Rc<RefCell<Cell>>>,
    pub dimensions: (isize, isize),
}

impl MutableGrid {
    pub fn iter(&self) -> impl Iterator<Item = Rc<RefCell<Cell>>> + '_ {
        self.cells.iter().cloned()
    }

    pub fn find_index(&self, value: &char) -> Result<GridIndex, ()> {
        for cell in &self.cells {
            if cell.borrow().value == *value {
                return Ok(cell.borrow().index.clone());
            }
        }
        Err(())
    }

    pub fn get_cell(&self, index: &GridIndex) -> Option<Rc<RefCell<Cell>>> {
        self.get_cell_by_index(&(index.x, index.y))
    }

    pub fn get_cell_by_index(&self, index: &(isize, isize)) -> Option<Rc<RefCell<Cell>>> {
        // Return None if the index is not in the grid.
        if index.0 < 0
            || index.0 >= self.dimensions.0
            || index.1 < 0
            || index.1 >= self.dimensions.1
        {
            return None;
        }

        let vec_index: usize = (index.0 * self.dimensions.1 + index.1).try_into().unwrap();
        Some(Rc::clone(&self.cells[vec_index]))
    }

    pub fn move_from_cell(&self, index: &GridIndex, direction: &Direction) -> Option<Rc<RefCell<Cell>>> {
        match direction {
            Direction::UP => self.get_cell_by_index(&(index.x - 1, index.y)),
            Direction::DOWN => self.get_cell_by_index(&(index.x + 1, index.y)),
            Direction::RIGHT => self.get_cell_by_index(&(index.x, index.y + 1)),
            Direction::LEFT => self.get_cell_by_index(&(index.x, index.y - 1)),
        }
    }
}

impl FromStr for MutableGrid {
    type Err = ();

    fn from_str(input: &str) -> Result<MutableGrid, Self::Err> {
        let mut cells = Vec::new();

        let mut max_column = 0;
        let mut row_index = 0;
        for line in input.split("\n") {
            let mut column_index = 0;
            for char_ in line.chars() {
                cells.push(Rc::new(RefCell::new(Cell {
                    index: GridIndex {
                        x: row_index,
                        y: column_index,
                    },
                    value: char_,
                })));
                column_index += 1;
            }
            max_column = column_index;
            row_index += 1;
        }

        let dimensions = (row_index, max_column);
        Ok(MutableGrid { cells, dimensions })
    }
}
