use crate::days_module::day::Day;

pub struct Day02 {}

fn is_safe<'a, I>(series: I, skip: Option<isize>) -> bool
where
    I: Iterator<Item = &'a isize>,
{
    let mut previous;
    let mut current = -1;
    let mut iterator = 0;
    for number in series {
        let skip = iterator == skip.unwrap_or(-1);

        iterator += 1;

        if skip {
            continue;
        }

        previous = current;
        current = *number;

        if previous == -1 {
            continue;
        }

        if current > previous || previous - current > 3 || current == previous {
            return false;
        }
    }
    true
}

impl Day for Day02 {
    fn get_id(&self) -> String {
        "day_02".to_string()
    }

    fn part_a(&self, input: &String) -> String {
        let input_iterator = input.split("\n");
        let mut safe_count = 0;
        for line in input_iterator {
            let integer_series: Vec<isize> = line
                .split(" ")
                .map(|x| x.parse::<isize>().unwrap())
                .collect();

            if is_safe(integer_series.iter(), None) || is_safe(integer_series.iter().rev(), None) {
                safe_count += 1;
            }
        }
        safe_count.to_string()
    }

    fn part_b(&self, input: &String) -> String {
        let input_iterator = input.split("\n");
        let mut safe_count = 0;
        for line in input_iterator {
            let integer_series: Vec<isize> = line
                .split(" ")
                .map(|x| x.parse::<isize>().unwrap())
                .collect();

            for i in 0..=integer_series.len() {
                if is_safe(integer_series.iter(), Some(i as isize))
                    || is_safe(integer_series.iter().rev(), Some(i as isize))
                {
                    safe_count += 1;
                    break;
                }
            }
        }
        safe_count.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_a() -> Result<(), String> {
        Day02 {}.test_day_part(&'a')
    }

    #[test]
    fn test_day_b() -> Result<(), String> {
        Day02 {}.test_day_part(&'b')
    }
}
