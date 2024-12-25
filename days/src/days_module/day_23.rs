use crate::days_module::day::Day;
use std::collections::{HashMap, HashSet};

pub struct Day23 {}

impl Day for Day23 {
    fn get_id(&self) -> String {
        "day_23".to_string()
    }

    fn get_index(&self) -> u8 {
        23
    }
    fn part_a(&self, input: &String) -> String {
        let mut edge_map = HashMap::new();
        for line in input.lines() {
            let mut split = line.split("-");
            let a = split.next().unwrap();
            let b = split.next().unwrap();

            let set_out_a = edge_map.entry(a).or_insert(HashSet::new());
            set_out_a.insert(b);

            let set_out_b = edge_map.entry(b).or_insert(HashSet::new());
            set_out_b.insert(a);
        }

        let mut trios = HashSet::new();

        for (computer, out_edges) in edge_map.iter() {
            if !computer.starts_with('t') {
                continue;
            }

            for out_computer in out_edges {
                for third_computer in out_edges.intersection(edge_map.get(out_computer).unwrap()) {
                    let mut trio = vec![
                        computer.to_owned(),
                        out_computer.to_owned(),
                        third_computer.to_owned(),
                    ];
                    trio.sort();
                    trios.insert(trio);
                }
            }
        }

        trios.len().to_string()
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
        Day23 {}.test_day_part(&'a')
    }

    #[test]
    fn test_day_b() -> Result<(), String> {
        Day23 {}.test_day_part(&'b')
    }
}
