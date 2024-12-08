use crate::days_module::day::Day;
use helpers::ints_from_string;
use rayon::prelude::*;

pub struct Day07 {}

fn concat(a: isize, b: isize) -> isize {
    let ab_str = a.to_string() + &*b.to_string();
    ab_str.parse::<isize>().unwrap()
}

fn can_be_made_true(
    target: &isize,
    memory: isize,
    numbers: &[isize],
    include_concatenate: bool,
) -> bool {
    // Prune.
    if memory > *target {
        return false;
    }

    // Assume left-to-right evaluation.
    if numbers.len() == 0 {
        if *target == memory {
            true
        } else {
            false
        }
    } else {
        if include_concatenate {
            can_be_made_true(
                target,
                memory * numbers[0],
                &numbers[1..],
                include_concatenate,
            ) || can_be_made_true(
                target,
                memory + numbers[0],
                &numbers[1..],
                include_concatenate,
            ) || can_be_made_true(
                target,
                concat(memory, numbers[0]),
                &numbers[1..],
                include_concatenate,
            )
        } else {
            can_be_made_true(
                target,
                memory * numbers[0],
                &numbers[1..],
                include_concatenate,
            ) || can_be_made_true(
                target,
                memory + numbers[0],
                &numbers[1..],
                include_concatenate,
            )
        }
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
                match can_be_made_true(&ints[0], ints[1].clone(), &ints[2..], false) {
                    false => 0,
                    true => ints[0],
                }
            })
            .sum::<isize>()
            .to_string()
    }

    fn part_b(&self, input: &String) -> String {
        input
            .par_lines()
            .map(|l| {
                let ints = ints_from_string(l.replace(": ", " ").trim());
                match can_be_made_true(&ints[0], ints[1].clone(), &ints[2..], true) {
                    false => 0,
                    true => ints[0],
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
