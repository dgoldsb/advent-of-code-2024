use crate::days_module::day::Day;
use helpers::ints_from_string;
use std::path::Component::ParentDir;

pub struct Day07 {}

fn concat(a: isize, b: isize) -> isize {
    let ab_str = a.to_string() + &*b.to_string();
    ab_str.parse::<isize>().unwrap()
}

fn find_combinations(
    target: &isize,
    memory: isize,
    numbers: &[isize],
    include_concatenate: bool,
) -> usize {
    // Assume left-to-right evaluation.
    if numbers.len() == 0 {
        if *target == memory {
            1
        } else {
            0
        }
    } else {
        let mut combinations = find_combinations(
            target,
            memory * numbers[0],
            &numbers[1..],
            include_concatenate,
        ) + find_combinations(
            target,
            memory + numbers[0],
            &numbers[1..],
            include_concatenate,
        );
        if include_concatenate {
            combinations += find_combinations(
                target,
                concat(memory, numbers[0]),
                &numbers[1..],
                include_concatenate,
            );
        }
        combinations
    }
}

impl Day for Day07 {
    fn get_id(&self) -> String {
        "day_07".to_string()
    }

    fn get_index(&self) -> u8 {
        7
    }
    fn part_a(&self, input: &String) -> String {
        input
            .lines()
            .map(|l| {
                let ints = ints_from_string(l.replace(": ", " ").trim());
                let combinations = find_combinations(&ints[0], ints[1].clone(), &ints[2..], false);
                match combinations {
                    0 => 0,
                    _ => ints[0],
                }
            })
            .sum::<isize>()
            .to_string()
    }

    fn part_b(&self, input: &String) -> String {
        input
            .lines()
            .map(|l| {
                let ints = ints_from_string(l.replace(": ", " ").trim());
                let combinations = find_combinations(&ints[0], ints[1].clone(), &ints[2..], true);
                match combinations {
                    0 => 0,
                    _ => ints[0],
                }
            })
            .sum::<isize>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_a() -> Result<(), String> {
        Day07 {}.test_day_part(&'a')
    }

    #[test]
    fn test_day_b() -> Result<(), String> {
        Day07 {}.test_day_part(&'b')
    }
}
