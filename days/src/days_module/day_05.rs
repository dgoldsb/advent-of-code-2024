use crate::days_module::day::Day;
use std::collections::{HashMap, HashSet};

pub struct Day05 {}

fn parse_rules(input: &str) -> Vec<(usize, usize)> {
    let mut rules = Vec::new();
    for line in input.lines() {
        let mut split = line.split("|");
        let before = split.next().unwrap();
        let after = split.next().unwrap();
        rules.push((after.parse().unwrap(), before.parse().unwrap()));
    }
    rules
}

fn is_valid_update(update: &Vec<usize>, rules: &Vec<(usize, usize)>) -> bool {
    let mut seen_set = HashSet::new();
    seen_set.insert(usize::MAX);

    // Add keys that will not occur.
    for rule in rules {
        if !update.contains(&rule.1) {
            seen_set.insert(rule.1);
        }
    }

    for page in update {
        for rule in rules {
            if rule.0 == *page && !seen_set.contains(&rule.1) {
                return false;
            }
        }

        seen_set.insert(*page);
    }

    true
}

impl Day for Day05 {
    fn get_id(&self) -> String {
        "day_05".to_string()
    }

    fn get_index(&self) -> u8 {
        5
    }
    fn part_a(&self, input: &String) -> String {
        let mut iterator = input.split("\n\n");
        let rules = parse_rules(iterator.next().unwrap());

        let mut count = 0;

        for raw_update in iterator.next().unwrap().lines() {
            let update = raw_update
                .split(",")
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            if is_valid_update(&update, &rules) {
                count += update[update.len() / 2];
            }
        }

        count.to_string()
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
        Day05 {}.test_day_part(&'a')
    }

    #[test]
    fn test_day_b() -> Result<(), String> {
        Day05 {}.test_day_part(&'b')
    }
}
