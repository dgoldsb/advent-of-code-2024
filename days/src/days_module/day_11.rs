use crate::days_module::day::Day;
use std::collections::HashMap;

pub struct Day11 {}

fn solve(blinks: usize, stone: usize, memory: &mut HashMap<(usize, usize), usize>) -> usize {
    if blinks == 0 {
        return 1;
    }

    if memory.contains_key(&(stone, blinks)) {
        return memory.get(&(stone, blinks)).unwrap().clone();
    }

    let mut result: usize;
    let stone_string = stone.to_string();

    if stone == 0 {
        result = solve(blinks - 1, 1, memory);
    } else if stone_string.len() % 2 == 0 {
        let first_stone = stone_string[0..stone.to_string().len() / 2]
            .parse::<usize>()
            .unwrap();
        let second_stone = stone_string[stone.to_string().len() / 2..stone_string.len()]
            .parse::<usize>()
            .unwrap();
        result = solve(blinks - 1, first_stone, memory) + solve(blinks - 1, second_stone, memory);
    } else {
        result = solve(blinks - 1, stone * 2024, memory);
    }

    memory.insert((stone, blinks), result.clone());
    result
}

impl Day for Day11 {
    fn get_id(&self) -> String {
        "day_11".to_string()
    }

    fn get_index(&self) -> u8 {
        11
    }
    fn part_a(&self, input: &String) -> String {
        let mut memory = HashMap::new();
        input
            .split(' ')
            .map(|s| solve(25, s.parse::<usize>().unwrap(), &mut memory))
            .sum::<usize>()
            .to_string()
    }

    fn part_b(&self, input: &String) -> String {
        let mut memory = HashMap::new();
        input
            .split(' ')
            .map(|s| solve(75, s.parse::<usize>().unwrap(), &mut memory))
            .sum::<usize>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_a() -> Result<(), String> {
        Day11 {}.test_day_part(&'a')
    }

    #[test]
    fn test_day_b() -> Result<(), String> {
        Day11 {}.test_day_part(&'b')
    }
}
