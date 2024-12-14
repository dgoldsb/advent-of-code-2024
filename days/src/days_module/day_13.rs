use crate::days_module::day::Day;
use helpers::{find_numbers, gauss_jordan};

fn find_minimum(
    a_delta: &(isize, isize),
    b_delta: &(isize, isize),
    target: &(isize, isize),
) -> usize {
    let result = gauss_jordan(vec![
        vec![a_delta.0 as f64, b_delta.0 as f64, target.0 as f64],
        vec![a_delta.1 as f64, b_delta.1 as f64, target.1 as f64],
    ])
    .unwrap();

    if (result[0].round() - result[0]).abs() > 0.0001
        || (result[1].round() - result[1]).abs() > 0.0001
    {
        return 0;
    }

    result[0].round() as usize * 3 + result[1].round() as usize
}

pub struct Day13 {}

impl Day for Day13 {
    fn get_id(&self) -> String {
        "day_13".to_string()
    }

    fn get_index(&self) -> u8 {
        13
    }

    fn part_a(&self, input: &String) -> String {
        let mut numbers = find_numbers(input);
        numbers.reverse();
        let mut result = 0;
        while !numbers.is_empty() {
            let a_delta = (numbers.pop().unwrap(), numbers.pop().unwrap());
            let b_delta = (numbers.pop().unwrap(), numbers.pop().unwrap());
            let target = (numbers.pop().unwrap(), numbers.pop().unwrap());
            result += find_minimum(&a_delta, &b_delta, &target);
        }

        result.to_string()
    }

    fn part_b(&self, input: &String) -> String {
        let mut numbers = find_numbers(input);
        numbers.reverse();
        let mut result = 0;
        while !numbers.is_empty() {
            let a_delta = (numbers.pop().unwrap(), numbers.pop().unwrap());
            let b_delta = (numbers.pop().unwrap(), numbers.pop().unwrap());
            let target = (
                numbers.pop().unwrap() + 10000000000000,
                numbers.pop().unwrap() + 10000000000000,
            );
            result += find_minimum(&a_delta, &b_delta, &target);
        }

        result.to_string()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_day_a() -> Result<(), String> {
        Day13 {}.test_day_part(&'a')
    }

    #[test]
    fn test_day_b() -> Result<(), String> {
        Day13 {}.test_day_part(&'b')
    }
}
