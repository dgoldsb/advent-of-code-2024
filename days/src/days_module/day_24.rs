use crate::days_module::day::Day;

pub struct Day24 {}

impl Day for Day24 {
    fn get_id(&self) -> String {
        "day_24".to_string()
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
        Day24 {}.test_day_part(&'a')
    }

    #[test]
    fn test_day_b() -> Result<(), String> {
        Day24 {}.test_day_part(&'b')
    }
}
