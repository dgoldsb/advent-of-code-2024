use crate::days_module::day::Day;
use std::collections::HashMap;

pub struct Day24 {}

struct Formula {
    a: String,
    b: String,
    operation: String,
    out: String,
}

fn try_solve(formula: &Formula, state: &HashMap<String, bool>) -> Option<bool> {
    if !(state.contains_key(&formula.a) && state.contains_key(&formula.b)) {
        return None;
    }

    let state_a = *state.get(&formula.a).unwrap();
    let state_b = *state.get(&formula.b).unwrap();

    let result = match formula.operation.as_str() {
        "AND" => state_a && state_b,
        "OR" => state_a || state_b,
        "XOR" => state_a == !state_b,
        _ => panic!(),
    };

    Some(result)
}

impl Day for Day24 {
    fn get_id(&self) -> String {
        "day_24".to_string()
    }

    fn get_index(&self) -> u8 {
        24
    }
    fn part_a(&self, input: &String) -> String {
        let mut input_iter = input.split("\n\n");
        let mut state = input_iter
            .next()
            .unwrap()
            .lines()
            .map(|x| {
                (
                    x.split_once(": ").unwrap().0.to_string(),
                    x.split_once(": ").unwrap().1 == "1",
                )
            })
            .collect::<HashMap<String, bool>>();
        let formulae = input_iter
            .next()
            .unwrap()
            .lines()
            .map(|x| {
                x.replace(" -> ", " ")
                    .split(" ")
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
            })
            .map(|v| Formula {
                a: v[0].to_string(),
                b: v[2].to_string(),
                operation: v[1].to_string(),
                out: v[3].to_string(),
            })
            .map(|f| (f.out.clone(), f))
            .collect::<HashMap<String, Formula>>();

        // Resolve formulae until nothing is left.
        let mut changed = true;
        while changed {
            changed = false;

            for (_, formula) in formulae.iter() {
                if !state.contains_key(&formula.out) {
                    if let Some(result) = try_solve(formula, &state) {
                        changed = true;
                        state.insert(formula.out.clone(), result);
                    }
                }
            }
        }

        let mut z_keys = state
            .keys()
            .cloned()
            .filter(|k| k.starts_with('z'))
            .collect::<Vec<String>>();
        z_keys.sort();
        z_keys.reverse();
        let binary_string = z_keys
            .iter()
            .map(|k| if *state.get(k).unwrap() { '1' } else { '0' })
            .collect::<String>();
        usize::from_str_radix(&binary_string, 2)
            .unwrap()
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
        Day24 {}.test_day_part(&'a')
    }

    #[test]
    fn test_day_b() -> Result<(), String> {
        Day24 {}.test_day_part(&'b')
    }
}
