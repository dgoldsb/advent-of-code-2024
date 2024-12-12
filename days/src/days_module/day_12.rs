use crate::days_module::day::Day;
use helpers::grid::grid::Grid;
use helpers::grid::grid_index::GridIndex;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

pub struct Day12 {}

impl Day for Day12 {
    fn get_id(&self) -> String {
        "day_12".to_string()
    }

    fn get_index(&self) -> u8 {
        12
    }

    fn part_a(&self, input: &String) -> String {
        // Find regions.
        let grid = Grid::from_str(input).unwrap();
        let mut seen_set = HashSet::new();
        let mut regions: HashMap<GridIndex, HashSet<GridIndex>> = HashMap::new();

        for cell in grid.iter() {
            if seen_set.contains(&cell.index) {
                continue;
            }

            let mut region = HashSet::new();

            let mut stack = vec![cell.index];
            while let Some(i) = stack.pop() {
                if seen_set.contains(&i) {
                    continue;
                }

                if grid.get_cell(&i).is_none() || grid.get_cell(&i).unwrap().value != cell.value {
                    continue;
                }

                seen_set.insert(i);
                region.insert(i);

                for new_index in i.neumann_neighborhood() {
                    stack.push(new_index);
                }
            }

            regions.insert(cell.index, region);
        }

        // Score regions.
        let mut result = 0;

        for region in regions.values() {
            let mut perimeter = 0;

            for index in region {
                for neighbor in index.neumann_neighborhood() {
                    if !region.contains(&neighbor) {
                        perimeter += 1;
                    }
                }
            }

            result += perimeter * region.len();
        }

        result.to_string()
    }

    fn part_b(&self, input: &String) -> String {
        // Find regions.
        let grid = Grid::from_str(input).unwrap();
        let mut seen_set = HashSet::new();
        let mut regions: HashMap<GridIndex, HashSet<GridIndex>> = HashMap::new();

        for cell in grid.iter() {
            if seen_set.contains(&cell.index) {
                continue;
            }

            let mut region = HashSet::new();

            let mut stack = vec![cell.index];
            while let Some(i) = stack.pop() {
                if seen_set.contains(&i) {
                    continue;
                }

                if grid.get_cell(&i).is_none() || grid.get_cell(&i).unwrap().value != cell.value {
                    continue;
                }

                seen_set.insert(i);
                region.insert(i);

                for new_index in i.neumann_neighborhood() {
                    stack.push(new_index);
                }
            }

            regions.insert(cell.index, region);
        }

        // Score regions with bulk discount
        let mut result = 0;

        for (start, region) in regions.iter() {
            let mut side_count = 0;

            // Verify that this lies on the border
            let mut found = false;
            for neighbor in start.neumann_neighborhood() {
                found = found || !region.contains(&neighbor);
            }
            if !found {
                panic!("sdflkjskfjdkls")
            }

            // Find the x-range to scan.
            let mut x_min = isize::MAX;
            let mut x_max = isize::MIN;

            for index in region {
                x_min = min(index.x, x_min);
                x_max = max(index.x, x_max);
            }

            let mut starts_lag = HashSet::new();
            let mut ends_lag = HashSet::new();

            for x_slice in x_min..=x_max {
                let mut slice = region
                    .iter()
                    .filter(|i| i.x == x_slice)
                    .map(|i| i.y)
                    .collect::<Vec<isize>>();
                slice.sort();

                let mut starts = HashSet::new();
                let mut ends = HashSet::new();
                let mut last_i = slice[0] - 1;

                starts.insert(slice[0]);
                ends.insert(slice[slice.len() - 1]);
                for i in slice {
                    if i - 1 != last_i {
                        ends.insert(last_i); // end of range
                        starts.insert(i); // start of range
                    }
                    last_i = i;
                }

                // Read lag and find changes to the side count.
                side_count += starts.difference(&starts_lag).collect::<Vec<_>>().len();
                side_count += ends.difference(&ends_lag).collect::<Vec<_>>().len();

                // Set lag.
                starts_lag = starts;
                ends_lag = ends;
            }

            // Find the y-range to scan.
            let mut y_min = isize::MAX;
            let mut y_max = isize::MIN;

            for index in region {
                y_min = min(index.y, y_min);
                y_max = max(index.y, y_max);
            }

            let mut starts_lag = HashSet::new();
            let mut ends_lag = HashSet::new();

            for y_slice in y_min..=y_max {
                let mut slice = region
                    .iter()
                    .filter(|i| i.y == y_slice)
                    .map(|i| i.x)
                    .collect::<Vec<isize>>();
                slice.sort();

                let mut starts = HashSet::new();
                let mut ends: HashSet<isize> = HashSet::new();
                let mut last_i = slice[0] - 1;

                starts.insert(slice[0]);
                ends.insert(slice[slice.len() - 1]);
                for i in slice {
                    if i - 1 != last_i {
                        ends.insert(last_i); // end of range
                        starts.insert(i); // start of range
                    }
                    last_i = i;
                }

                // Read lag and find changes to the side count.
                side_count += starts.difference(&starts_lag).collect::<Vec<_>>().len();
                side_count += ends.difference(&ends_lag).collect::<Vec<_>>().len();

                // Set lag.
                starts_lag = starts;
                ends_lag = ends;
            }

            result += side_count * region.len();
        }

        result.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_a() -> Result<(), String> {
        Day12 {}.test_day_part(&'a')
    }

    #[test]
    fn test_day_b() -> Result<(), String> {
        Day12 {}.test_day_part(&'b')
    }
}
