use crate::days_module::day::Day;

pub struct Day03 {}

use regex::Regex;

impl Day for Day03 {
    fn get_id(&self) -> String {
        "day_03".to_string()
    }

    fn get_index(&self) -> u8 {
        3
    }

    fn part_a(&self, input: &String) -> String {
        let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
        let mut sum = 0;
        for m in re.captures_iter(input) {
            sum += m[1].parse::<i32>().unwrap() * m[2].parse::<i32>().unwrap();
        }
        sum.to_string()
    }

    fn part_b(&self, input: &String) -> String {
        let re = Regex::new(r"(mul|do|don't)\((\d*),?(\d*)\)").unwrap();
        let mut sum = 0;
        let mut active = true;
        for m in re.captures_iter(input) {
            match &m[1] {
                "do" => active = true,
                "don't" => active = false,
                "mul" => {
                    if active {
                        sum += m[2].parse::<i32>().unwrap() * m[3].parse::<i32>().unwrap();
                    }
                }
                _ => {
                    panic!("Unknown instruction");
                }
            }
        }
        sum.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_a() -> Result<(), String> {
        Day03 {}.test_day_part(&'a')
    }

    #[test]
    fn test_day_b() -> Result<(), String> {
        Day03 {}.test_day_part(&'b')
    }
}
