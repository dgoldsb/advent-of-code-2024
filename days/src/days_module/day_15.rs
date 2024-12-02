use crate::days_module::day::Day;

pub struct Day15 {}

impl Day for Day15 {
    fn get_id(&self) -> String {
        "day_15".to_string()
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
        Day15 {}.test_day_part(&'a')
    }

    #[test]
    fn test_day_b() -> Result<(), String> {
        Day15 {}.test_day_part(&'b')
    }
}
