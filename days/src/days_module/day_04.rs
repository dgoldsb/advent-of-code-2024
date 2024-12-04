use crate::days_module::day::Day;

pub struct Day04 {}

impl Day for Day04 {
    fn get_id(&self) -> String {
        "day_04".to_string()
    }

    fn get_index(&self) -> u8 {
        4
    }

    fn part_a(&self, input: &String) -> String {
        let input_bytes = input.as_bytes();
        let line_length: isize = input.find('\n').unwrap() as isize + 1;
        let mut count = 0;

        for delta in vec![
            1,
            -1,
            line_length,
            -line_length,
            line_length + 1,
            line_length - 1,
            -line_length + 1,
            -line_length - 1,
        ] {
            for i in 0..input.len() {
                let x = *input_bytes.get(i).unwrap_or(&0) as char;
                // Let negative isize overflow because YOLO.
                let m = *input_bytes
                    .get((i as isize + delta) as usize)
                    .unwrap_or(&0u8) as char;
                let a = *input_bytes
                    .get((i as isize + 2 * delta) as usize)
                    .unwrap_or(&0u8) as char;
                let s = *input_bytes
                    .get((i as isize + 3 * delta) as usize)
                    .unwrap_or(&0u8) as char;
                if x == 'X' && m == 'M' && a == 'A' && s == 'S' {
                    count += 1;
                }
            }
        }

        count.to_string()
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
        Day04 {}.test_day_part(&'a')
    }

    #[test]
    fn test_day_b() -> Result<(), String> {
        Day04 {}.test_day_part(&'b')
    }
}
