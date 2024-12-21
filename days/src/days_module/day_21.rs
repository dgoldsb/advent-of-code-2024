use crate::days_module::day::Day;
use lazy_static::lazy_static;
use std::collections::HashMap;

pub struct Day21 {}

/* Entering a button one layer up always ends on "A", so this is a checkpoint in finding the shortest sequence.

There is up to two "paths": x then y, y then x. Sometimes only one is possible, and one may be shorter.
Directional keypads have few transitions (20).
The final keypad has a manageable number of transitions (90).
I can "expand" the shortest transition for each layer.

(3, 9) -> '^^A'

(<, ^) -> '>^A'
(^, ^) -> 'A'
(^, A) -> '>A'

(<, >) -> '>>A'
(>, ^) -> '<^A'
(^, A) -> '>A'

And so forth, so if I build these two maps I can expand each button press.
For the two paths, either we multiply the two options (mean 2 * 2 * 2 = 8 paths to evaluate per press)...
Or we assume that the one that ends on ^ or > is fastest because it ends closer to the "A".
I think the branching is not crucial, because for the arrow keypads there is no relevant ambiguity (I think).

Also, something recursive? I think part 2 will add more layers of keypads.
*/

// TODO: The key is to realize that the optimal sequence is to either to move to the target first column and then finally to the row or vice versa, leaving only two possible sequences to consider, neither of which you can rule out locally.


lazy_static! {
    static ref DIRECTION_PAD_TRANSITIONS: HashMap<(char, char), Vec<char>> = HashMap::from([
        (('<', '^'), vec!['>', '^', 'A']),
        (('<', 'v'), vec!['>', 'A']),
        (('<', '>'), vec!['>', '>', 'A']),
        (('<', 'A'), vec!['>', '>', '^', 'A']),
        (('<', '<'), vec!['A']),
        (('v', '^'), vec!['^', 'A']),
        (('v', 'v'), vec!['A']),
        (('v', '>'), vec!['>', 'A']),
        (('v', 'A'), vec!['>', '^', 'A']),
        (('v', '<'), vec!['<', 'A']),
        (('>', '^'), vec!['<', '^', 'A']),
        (('>', 'v'), vec!['<', 'A']),
        (('>', '>'), vec!['A']),
        (('>', 'A'), vec!['^', 'A']),
        (('>', '<'), vec!['<', '<', 'A']),
        (('^', '^'), vec!['A']),
        (('^', 'v'), vec!['v', 'A']),
        (('^', '>'), vec!['v', '>', 'A']),
        (('^', 'A'), vec!['>', 'A']),
        (('^', '<'), vec!['v', '<', 'A']),
        (('A', '^'), vec!['<', 'A']),
        (('A', 'v'), vec!['<', 'v', 'A']),
        (('A', '>'), vec!['v', 'A']),
        (('A', 'A'), vec!['A']),
        (('A', '<'), vec!['v', '<', '<', 'A']),
    ]);
}

lazy_static! {
    static ref NUMBER_COORDINATES: HashMap<char, (isize, isize)> = HashMap::from([
        ('7', (0, 0)),
        ('8', (1, 0)),
        ('9', (2, 0)),
        ('4', (0, 1)),
        ('5', (1, 1)),
        ('6', (2, 1)),
        ('1', (0, 2)),
        ('2', (1, 2)),
        ('3', (2, 2)),
        ('0', (1, 3)),
        ('A', (2, 3)),
    ]);
}

fn resolve_robot_chain(target_without_start: &Vec<char>, layers_left: usize) -> Vec<char> {
    println!("Target for robot {}: {:?}", layers_left, target_without_start);
    let mut target = vec!['A'];
    target.extend_from_slice(target_without_start);
    let mut result = Vec::new();

    // Check what moves this robot needs to do.
    for char_pair in target.windows(2) {
        result.extend(&DIRECTION_PAD_TRANSITIONS[&(char_pair[0], char_pair[1])]);
    }

    // If necessary, go a layer deeper.
    if layers_left != 0 {
        resolve_robot_chain(
            &result,
            layers_left - 1,
        )
    } else {
        result
    }
}

fn enter_code(code: &str) -> Vec<char> {
    let code = "A".to_owned() + code;
    let mut request = Vec::new();

    for char_pair in code.chars().collect::<Vec<_>>().windows(2) {
        let from = NUMBER_COORDINATES[&char_pair[0]];
        let to = NUMBER_COORDINATES[&char_pair[1]];
        let x_delta = to.0 - from.0;
        let y_delta = to.1 - from.1;

        // First right.
        if x_delta > 0 {
            for _ in 0..x_delta.abs() {
                request.push('>');
            }
        }

        // Then up.
        if y_delta < 0 {
            for _ in 0..y_delta.abs() {
                request.push('^');
            }
        }

        // Then left.
        if x_delta < 0 {
            for _ in 0..x_delta.abs() {
                request.push('<');
            }
        }
        // Last down.
        if y_delta > 0 {
            for _ in 0..y_delta.abs() {
                request.push('v');
            }
        }

        request.push('A');
    }

    let result = resolve_robot_chain(&request, 1);
    println!("Final result: {:?}", result);
    result
}

impl Day for Day21 {
    fn get_id(&self) -> String {
        "day_21".to_string()
    }

    fn get_index(&self) -> u8 {
        21
    }
    fn part_a(&self, input: &String) -> String {
        input
            .lines()
            .map(|line| enter_code(line).len() * line[0..(line.len() - 1)].parse::<usize>().unwrap())
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
        Day21 {}.test_day_part(&'a')
    }

    #[test]
    fn test_day_b() -> Result<(), String> {
        Day21 {}.test_day_part(&'b')
    }
}
