use crate::days_module::day::Day;

pub struct Day22 {}

fn mix_into_secret(value: usize, secret: usize) -> usize {
    value ^ secret
}

fn prune_secret(secret: usize) -> usize {
    secret % 16777216
}

fn evolve_secret(secret: usize) -> usize {
    let result_1 = prune_secret(mix_into_secret(secret, secret * 64));
    let result_2 = prune_secret(mix_into_secret(result_1, result_1 / 32));
    let result_3 = prune_secret(mix_into_secret(result_2, result_2 * 2048));
    result_3
}

fn nth_secret(secret: usize, n: usize) -> usize {
    let mut output = secret;
    for _ in 0..n {
        output = evolve_secret(output);
    }
    output
}

impl Day for Day22 {
    fn get_id(&self) -> String {
        "day_22".to_string()
    }

    fn get_index(&self) -> u8 {
        22
    }
    fn part_a(&self, input: &String) -> String {
        input
            .lines()
            .map(|line| nth_secret(line.parse::<usize>().unwrap(), 2000))
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
    fn test_mix() -> Result<(), String> {
        if mix_into_secret(42, 15) == 37 {
            Ok(())
        } else {
            Err(String::from("Wrong result"))
        }
    }

    #[test]
    fn test_prune() -> Result<(), String> {
        if prune_secret(100000000) == 16113920 {
            Ok(())
        } else {
            Err(String::from("Wrong result"))
        }
    }

    #[test]
    fn test_evolve() -> Result<(), String> {
        if evolve_secret(123) == 15887950 {
            Ok(())
        } else {
            Err(String::from("Wrong result"))
        }
    }

    #[test]
    fn test_evolve_nth() -> Result<(), String> {
        if nth_secret(123, 4) == 704524 {
            Ok(())
        } else {
            Err(String::from("Wrong result"))
        }
    }

    #[test]
    fn test_day_a() -> Result<(), String> {
        Day22 {}.test_day_part(&'a')
    }

    #[test]
    fn test_day_b() -> Result<(), String> {
        Day22 {}.test_day_part(&'b')
    }
}
