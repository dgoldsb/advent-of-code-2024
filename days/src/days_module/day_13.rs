use crate::days_module::day::Day;
use helpers::gauss_jordan;

use lazy_static::lazy_static;
use regex::Regex;

fn find_numbers(s: &str) -> Vec<usize> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\d+").unwrap();
    }

    RE.find_iter(s)
        .filter_map(|digits| digits.as_str().parse().ok())
        .collect()
}

pub struct Day13 {}

fn find_minimum(
    a_delta: &(usize, usize),
    b_delta: &(usize, usize),
    target: &(usize, usize),
) -> usize {
    let result = gauss_jordan(vec![
        vec![a_delta.0 as f64, b_delta.0 as f64, target.0 as f64],
        vec![a_delta.1 as f64, b_delta.1 as f64, target.1 as f64],
    ])
    .unwrap();

    if (result[0].round() - result[0]).abs() > 0.000001
        || (result[1].round() - result[1]).abs() > 0.000001
    {
        return 0;
    }

    result[0].round() as usize * 3 + result[1].round() as usize
}

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
        "".to_string()
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
