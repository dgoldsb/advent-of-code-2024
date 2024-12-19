use crate::days_module::day::Day;
use std::collections::HashSet;

pub struct Day19 {}

fn count_possible(patterns: &Vec<String>, designs: &Vec<String>) -> usize {
    let mut seen = HashSet::new();
    let mut stack = patterns.clone();

    while let Some(tail) = stack.pop() {
        let found = designs.iter().any(|p| p == &tail);

        if found {
            seen.insert(tail.clone());
        }

        let valid_tail = designs.iter().any(|p| {
            p.len() >= tail.len() && p[(p.len() - tail.len())..p.len()] == tail[0..tail.len()]
        });

        if valid_tail {
            for pattern in patterns {
                let extended_tail = pattern.to_owned() + &tail;
                if !seen.contains(&extended_tail) {
                    seen.insert(extended_tail.clone());
                    stack.push(extended_tail);
                }
            }
        }
    }

    designs.iter().filter(|&s| seen.contains(s)).count()
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

    return (patterns, designs);
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
        count_possible(&patterns, &designs).to_string()
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
        Day19 {}.test_day_part(&'a')
    }

    #[test]
    fn test_day_b() -> Result<(), String> {
        Day19 {}.test_day_part(&'b')
    }
}
