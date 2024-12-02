use crate::days_module::day::Day;
use std::collections::HashMap;

pub struct Day01 {}

impl Day for Day01 {
    fn get_id(&self) -> String {
        "day_01".to_string()
    }

    fn part_a(&self, input: &String) -> String {
        let mut input_iterator = input.split("\n");

        let mut series_a = Vec::new();
        let mut series_b = Vec::new();

        for line in input_iterator {
            let mut number_iterator = line.split("   ");
            series_a.push(number_iterator.next().unwrap().parse::<i32>().unwrap());
            series_b.push(number_iterator.next().unwrap().parse::<i32>().unwrap());
        }

        series_a.sort();
        series_b.sort();

        series_a
            .iter()
            .zip(series_b.iter())
            .map(|(a, b)| (a - b).abs())
            .sum::<i32>()
            .to_string()
    }

    fn part_b(&self, input: &String) -> String {
        let mut input_iterator = input.split("\n");

        let mut map_a = HashMap::new();
        let mut map_b = HashMap::new();

        for line in input_iterator {
            let mut number_iterator = line.split("   ");

            let value_a: i32 = number_iterator.next().unwrap().parse::<i32>().unwrap();
            match map_a.get(&value_a) {
                Some(v) => {
                    map_a.insert(value_a, v + 1);
                }
                None => {
                    map_a.insert(value_a, 1);
                }
            }
            let value_b: i32 = number_iterator.next().unwrap().parse::<i32>().unwrap();
            match map_b.get(&value_b) {
                Some(v) => {
                    map_b.insert(value_b, v + 1);
                }
                None => {
                    map_b.insert(value_b, 1);
                }
            }
        }

        map_a
            .iter()
            .map(|(k, v)| k * v * map_b.get(k).unwrap_or(&0))
            .sum::<i32>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_a() -> Result<(), String> {
        Day01 {}.test_day_part(&'a')
    }

    #[test]
    fn test_day_b() -> Result<(), String> {
        Day01 {}.test_day_part(&'b')
    }
}
