use crate::days_module::day::Day;

pub struct Day09 {}

#[derive(Clone)]
struct File {
    size: usize,
    value: usize,
}

struct MemoryBlock {
    size: usize,
    content: Option<File>,
}

impl MemoryBlock {
    fn expand(&self) -> Vec<usize> {
        let mut result = Vec::new();

        if self.content.is_some() {
            let content = self.content.clone().unwrap();

            for _ in 0..content.size {
                result.push(content.value);
            }
        }

        while result.len() < self.size {
            result.push(0);
        }

        result
    }
}

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

fn decompress_into_memory(input: &str) -> Vec<MemoryBlock> {
    let mut uncompressed = Vec::new();
    let mut value = 0;
    let mut block = true;

    for char in input.chars() {
        let size = char.to_digit(10).unwrap() as usize;
        let content = Some(File { size, value });
        let memory_block = if block {
            value += 1;
            MemoryBlock { size, content }
        } else {
            MemoryBlock {
                size,
                content: None,
            }
        };

        uncompressed.push(memory_block);
        block = !block;
    }

    uncompressed
}

fn shuffle_memory(input: &mut Vec<MemoryBlock>) -> &Vec<MemoryBlock> {
    let mut end_index = input.len() - 1;
    while end_index > 0 {
        let end_value = &input[end_index];

        if end_value.content.is_none() {
            end_index -= 1;
            continue;
        }

        let mut start_index = 0;
        while start_index < end_index {
            let start_value = &input[start_index];

            if start_value.content.is_some() {
                start_index += 1;
                continue;
            } else if start_value.size >= end_value.size {
                input[start_index].content = end_value.content.clone();
                input[end_index].content = None;
                break;
            } else {
                start_index += 1;
            }
        }
        end_index -= 1;
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
        let mut uncompressed = decompress_into_memory(input);
        let shuffled = shuffle_memory(&mut uncompressed);

        let debug = shuffled
            .iter()
            .map(|m| m.expand())
            .flatten()
            .collect::<Vec<usize>>();

        shuffled
            .iter()
            .map(|m| m.expand())
            .flatten()
            .enumerate()
            .map(|(i, o)| i * o)
            .sum::<usize>()
            .to_string()
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
