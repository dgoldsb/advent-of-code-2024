use crate::days_module::day::Day;
use std::collections::{HashMap, HashSet};

pub struct Day23 {}

fn get_edge_map(input: &String) -> HashMap<&str, HashSet<&str>> {
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
    edge_map
}

fn serialize(set: &HashSet<&str>) -> String {
    let mut vec = set
        .into_iter()
        .map(|i| i.to_string())
        .collect::<Vec<String>>();
    vec.sort();
    vec.join(",")
}

fn explore_biggest_set(edge_map: &HashMap<&str, HashSet<&str>>) -> Vec<String> {
    let mut biggest_set: HashSet<&str> = HashSet::new();
    let mut stack = edge_map
        .keys()
        .map(|k| (*k, HashSet::from([*k])))
        .collect::<Vec<(&str, HashSet<&str>)>>();
    let mut visited_sets = stack
        .clone()
        .iter()
        .map(|(_, s)| serialize(s))
        .collect::<HashSet<String>>();

    while let Some((n, s)) = stack.pop() {
        for out_node in edge_map.get(&n).unwrap() {
            if s.contains(out_node) {
                continue;
            }

            let out_node_set = edge_map.get(out_node).unwrap();

            if s.is_subset(out_node_set) {
                let mut new_set = s.clone();
                new_set.insert(out_node);

                if !visited_sets.contains(&serialize(&new_set)) {
                    stack.push((n, new_set.clone()));
                    visited_sets.insert(serialize(&new_set));
                    if new_set.len() > biggest_set.len() {
                        biggest_set = new_set.clone();
                    }
                }
            }
        }
    }

    biggest_set
        .into_iter()
        .map(|i| i.to_string())
        .collect::<Vec<String>>()
}

impl Day for Day23 {
    fn get_id(&self) -> String {
        "day_23".to_string()
    }

    fn get_index(&self) -> u8 {
        23
    }
    fn part_a(&self, input: &String) -> String {
        let edge_map = get_edge_map(input);
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
        let edge_map = get_edge_map(input);
        let mut biggest_set = explore_biggest_set(&edge_map);
        biggest_set.sort();
        biggest_set.join(",")
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
