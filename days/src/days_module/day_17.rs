use helpers::find_usize;

use crate::days_module::day::Day;

struct Computer {
    instruction_pointer: usize,
    a: usize,
    b: usize,
    c: usize,
}

impl Computer {
    fn find_combo(&self, operant: &usize) -> usize {
        match operant % 8 {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            i => {
                panic!("Received invalid Opcode {}", i);
            }
        }
    }

    fn apply_opcode(&mut self, opcode: &usize, operant: &usize) -> Option<usize> {
        match opcode {
            0 => {
                self.a = self.a
                    / 2_usize
                        .checked_pow(self.find_combo(operant) as u32)
                        .unwrap();
                self.instruction_pointer += 2;
            }
            1 => {
                self.b = self.b ^ operant;
                self.instruction_pointer += 2;
            }
            2 => {
                self.b = self.find_combo(operant) % 8;
                self.instruction_pointer += 2;
            }
            3 => {
                if self.a != 0 {
                    self.instruction_pointer = *operant;
                } else {
                    self.instruction_pointer += 2;
                }
            }
            4 => {
                self.b = self.b ^ self.c;
                self.instruction_pointer += 2;
            }
            5 => {
                self.instruction_pointer += 2;
                return Some(self.find_combo(operant) % 8);
            }
            6 => {
                self.b = self.a
                    / 2_usize
                        .checked_pow(self.find_combo(operant) as u32)
                        .unwrap();
                self.instruction_pointer += 2;
            }
            7 => {
                self.c = self.a
                    / 2_usize
                        .checked_pow(self.find_combo(operant) as u32)
                        .unwrap();
                self.instruction_pointer += 2;
            }
            i => {
                panic!("Received invalid Opcode {}", i);
            }
        }
        None
    }

    fn run(&mut self, program: Vec<usize>) -> String {
        let mut output = String::new();

        loop {
            if self.instruction_pointer >= program.len() {
                break;
            }

            let opcode = program[self.instruction_pointer];
            let operant = program[self.instruction_pointer + 1];
            let result = self.apply_opcode(&opcode, &operant);

            if result.is_some() {
                if output.len() > 0 {
                    output.push(',');
                }
                output.push_str(&result.unwrap().to_string());
            }
        }
        output
    }
}

pub struct Day17 {}

impl Day for Day17 {
    fn get_id(&self) -> String {
        "day_17".to_string()
    }

    fn get_index(&self) -> u8 {
        17
    }
    fn part_a(&self, input: &String) -> String {
        let numbers = find_usize(input);
        let mut numbers_iter = numbers.iter();

        let mut computer = Computer {
            instruction_pointer: 0,
            a: numbers_iter.next().unwrap().clone(),
            b: numbers_iter.next().unwrap().clone(),
            c: numbers_iter.next().unwrap().clone(),
        };
        let program = numbers_iter.map(|i| i.clone()).collect::<Vec<usize>>();

        computer.run(program)
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
