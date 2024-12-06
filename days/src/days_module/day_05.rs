use crate::days_module::day::Day;
use rayon::prelude::*;
use std::collections::HashSet;

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

fn is_valid_update(update: &Vec<usize>, rules: &Vec<(usize, usize)>) -> Option<(usize, usize)> {
    let mut seen_set = HashSet::new();
    seen_set.insert(usize::MAX);

    // Add keys that will not occur.
    for rule in rules {
        if !update.contains(&rule.1) {
            seen_set.insert(rule.1);
        }
    }

    let mut i = 0;
    for page in update {
        for rule in rules {
            if rule.0 == *page && !seen_set.contains(&rule.1) {
                return Some((i, update.iter().position(|&x| x == rule.1).unwrap()));
            }
        }

        seen_set.insert(*page);
        i += 1;
    }

    None
}

fn fix_update(update: Vec<usize>, rules: &Vec<(usize, usize)>) -> Vec<usize> {
    let mut new_update = update.clone();
    while is_valid_update(&new_update, rules) != None {
        let offending_index = is_valid_update(&new_update, rules).unwrap();
        new_update.swap(offending_index.0, offending_index.1);
    }

    new_update
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

        iterator
            .next()
            .unwrap()
            .lines()
            .map(|raw_update| {
                let update = raw_update
                    .split(",")
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<Vec<_>>();
                if is_valid_update(&update, &rules) == None {
                    update[update.len() / 2]
                } else {
                    0
                }
            })
            .sum::<usize>()
            .to_string()
    }

    fn part_b(&self, input: &String) -> String {
        let mut iterator = input.split("\n\n");
        let rules = parse_rules(iterator.next().unwrap());

        iterator
            .next()
            .unwrap()
            .par_lines()
            .map(|raw_update| {
                let update = raw_update
                    .split(",")
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<Vec<_>>();

                if is_valid_update(&update, &rules) == None {
                    0
                } else {
                    let fixed_update = fix_update(update, &rules);

                    if is_valid_update(&fixed_update, &rules) == None {
                        fixed_update[fixed_update.len() / 2]
                    } else {
                        0
                    }
                }
            })
            .sum::<usize>()
            .to_string()
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
