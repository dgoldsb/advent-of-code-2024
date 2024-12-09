use crate::days_module::day::Day;

pub struct Day09 {}

fn decompress(input: &str) -> Vec<Option<usize>> {
    let mut uncompressed = Vec::new();
    let mut block_index = 0;
    let mut block = true;

    for char in input.chars() {
        let length = char.to_digit(10).unwrap() as usize;
        for _ in 0..length {
            if block {
                uncompressed.push(Some(block_index));
            } else {
                uncompressed.push(None);
            }
        }

        if block {
            block_index += 1;
        }
        block = !block;
    }

    uncompressed
}

fn shuffle(input: &mut Vec<Option<usize>>) -> &Vec<Option<usize>> {
    let mut start_index = 0;
    let mut end_index = input.len() - 1;

    while start_index != end_index {
        match input[start_index] {
            Some(_) => start_index += 1,
            None => {
                let end_value = input[end_index];
                input[start_index] = end_value;
                input[end_index] = None;
                end_index -= 1;
            }
        }
    }

    input
}

impl Day for Day09 {
    fn get_id(&self) -> String {
        "day_09".to_string()
    }

    fn get_index(&self) -> u8 {
        9
    }

    fn part_a(&self, input: &String) -> String {
        let mut uncompressed = decompress(input);
        let shuffled = shuffle(&mut uncompressed);
        shuffled
            .iter()
            .filter(|o| o.is_some())
            .enumerate()
            .map(|(i, o)| i * o.unwrap())
            .sum::<usize>()
            .to_string()
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
        Day09 {}.test_day_part(&'a')
    }

    #[test]
    fn test_day_b() -> Result<(), String> {
        Day09 {}.test_day_part(&'b')
    }
}
