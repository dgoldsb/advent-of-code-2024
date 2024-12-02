use crate::days_module::day::Day;

pub struct Day22 {}

impl Day for Day22 {
    fn get_id(&self) -> String {
        "day_22".to_string()
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
        Day22 {}.test_day_part(&'a')
    }

    #[test]
    fn test_day_b() -> Result<(), String> {
        Day22 {}.test_day_part(&'b')
    }
}
