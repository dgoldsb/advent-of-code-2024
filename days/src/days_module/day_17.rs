use crate::days_module::day::Day;

pub struct Day17 {}

impl Day for Day17 {
    fn get_id(&self) -> String {
        "day_17".to_string()
    }

    fn get_index(&self) -> u8 {
        1
    }
    fn part_a(&self, input: &String) -> String {
        "".to_string()
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
        Day17 {}.test_day_part(&'a')
    }

    #[test]
    fn test_day_b() -> Result<(), String> {
        Day17 {}.test_day_part(&'b')
    }
}
