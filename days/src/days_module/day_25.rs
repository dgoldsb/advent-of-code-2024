use crate::days_module::day::Day;
use helpers::grid::grid::Grid;
use std::str::FromStr;

pub struct Day25 {}

impl Day for Day25 {
    fn get_id(&self) -> String {
        "day_25".to_string()
    }

    fn get_index(&self) -> u8 {
        25
    }
    fn part_a(&self, input: &String) -> String {
        let key_or_locks = input
            .split("\n\n")
            .map(|s| Grid::from_str(s).unwrap())
            .collect::<Vec<Grid>>();

        let mut counter = 0;
        for a in key_or_locks.iter() {
            for b in key_or_locks.iter() {
                let mut overlaps = false;
                for cell in a.iter() {
                    if cell.value == '#' && cell.value == b.get_cell(&cell.index).unwrap().value {
                        overlaps = true;
                        break;
                    }
                }
                if !overlaps {
                    counter += 1;
                }
            }
        }

        (counter / 2).to_string()
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
        Day25 {}.test_day_part(&'a')
    }

    #[test]
    fn test_day_b() -> Result<(), String> {
        Day25 {}.test_day_part(&'b')
    }
}
