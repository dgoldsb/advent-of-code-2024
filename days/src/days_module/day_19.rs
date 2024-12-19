use crate::days_module::day::Day;
use std::collections::{BinaryHeap, HashMap, HashSet};

pub struct Day19 {}

fn dijkstra(patterns: &Vec<String>, designs: &Vec<String>) -> HashMap<String, usize> {
    let max_length = designs.iter().map(|d| d.len()).max().unwrap();

    let mut visited_count: HashMap<String, usize> = HashMap::new();
    let mut open_set: HashSet<String> = HashSet::new();
    let mut max_heap: BinaryHeap<(usize, String)> = BinaryHeap::new();

    for pattern in patterns {
        max_heap.push((usize::MAX - pattern.len(), String::from(pattern)));
        open_set.insert(String::from(pattern));
        visited_count.insert(String::from(pattern), 1);
    }

    while !max_heap.is_empty() {
        let (_, head) = max_heap.pop().unwrap();
        let head_count = visited_count.get(&head).unwrap().clone();
        open_set.remove(&head);

        for pattern in patterns {
            let neighbor = head.to_owned() + &pattern;
            let new_key = !visited_count.contains_key(&neighbor);

            let can_match = designs
                .iter()
                .any(|design| design.starts_with(neighbor.as_str()));

            // TODO exit early
            if !can_match || neighbor.len() > max_length {
                continue;
            }

            if new_key && !open_set.contains(&neighbor) {
                max_heap.push((usize::MAX - neighbor.len(), neighbor.clone()));
                open_set.insert(neighbor.clone());
            }
            *visited_count.entry(neighbor).or_insert(0) += head_count;
        }
    }

    visited_count
}

fn parse(input: &str) -> (Vec<String>, Vec<String>) {
    let mut input_iter = input.split("\n\n");

    let patterns = input_iter
        .next()
        .unwrap()
        .split(", ")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let designs = input_iter
        .next()
        .unwrap()
        .split("\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    (patterns, designs)
}

impl Day for Day19 {
    fn get_id(&self) -> String {
        "day_19".to_string()
    }

    fn get_index(&self) -> u8 {
        19
    }

    fn part_a(&self, input: &String) -> String {
        let (patterns, designs) = parse(input);
        let map = dijkstra(&patterns, &designs);
        designs
            .iter()
            .filter(|d| map.contains_key(*d))
            .count()
            .to_string()
    }

    fn part_b(&self, input: &String) -> String {
        let (patterns, designs) = parse(input);
        let map = dijkstra(&patterns, &designs);
        designs
            .iter()
            .filter(|d| map.contains_key(*d))
            .map(|d| map.get(d).unwrap())
            .sum::<usize>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_a() -> Result<(), String> {
        Day19 {}.test_day_part(&'a')
    }

    #[test]
    fn test_day_b() -> Result<(), String> {
        Day19 {}.test_day_part(&'b')
    }
}
