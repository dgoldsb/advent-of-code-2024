use crate::days_module::day::Day;

pub struct Day11 {}
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Rc<RefCell<Node<T>>>>,
}

#[derive(Debug)]
struct DoublyLinkedList<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
    size: usize,
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            size: 0,
        }
    }

    pub fn push_back(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(Node {
            value,
            next: None,
            prev: self.tail.clone(),
        }));

        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_node.clone());
                self.tail = Some(new_node);
            }
            None => {
                // List is empty
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
        }

        self.size += 1;
    }
}

impl Day for Day11 {
    fn get_id(&self) -> String {
        "day_11".to_string()
    }

    fn get_index(&self) -> u8 {
        11
    }
    fn part_a(&self, input: &String) -> String {
        // Screams linked list.
        let mut stones = DoublyLinkedList::new();
        input
            .split(" ")
            .for_each(|x| stones.push_back(x.parse::<usize>().unwrap()));

        for _ in 0..25 {
            let mut current = stones.head.clone().unwrap();
            loop {
                let current_value = current.borrow().value;
                let next_node = current.borrow().next.clone();

                if current_value == 0 {
                    current.borrow_mut().value = 1;
                } else if current_value.to_string().len() % 2 == 0 {
                    let current_value_string = current_value.to_string();
                    let first_half = current_value_string[0..current_value_string.len() / 2]
                        .parse::<usize>()
                        .unwrap();
                    let second_half = current_value_string
                        [current_value_string.len() / 2..current_value_string.len()]
                        .parse::<usize>()
                        .unwrap();

                    let new = Node {
                        value: second_half,
                        next: current.borrow().next.clone(),
                        prev: current.borrow().prev.clone(),
                    };
                    current.borrow_mut().value = first_half;
                    current.borrow_mut().next = Some(Rc::new(RefCell::new(new)));
                    stones.size += 1;
                } else {
                    current.borrow_mut().value *= 2024;
                }

                if next_node.is_none() {
                    break;
                } else {
                    current = next_node.unwrap();
                }
            }
        }

        stones.size.to_string()
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
        Day11 {}.test_day_part(&'a')
    }

    #[test]
    fn test_day_b() -> Result<(), String> {
        Day11 {}.test_day_part(&'b')
    }
}
