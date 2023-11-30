use std::{cell::RefCell, rc::Rc, str::FromStr};

use async_trait::async_trait;
use common::{Answer, Solution};

pub struct Puzzle {}

impl Default for Puzzle {
    fn default() -> Self {
        Self {}
    }
}

type Link<T> = Option<Rc<RefCell<T>>>;
struct Digit {
    pub value: i64,
    next: Link<Digit>,
    prev: Link<Digit>,
}

impl FromStr for Digit {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            value: s.parse::<i64>().unwrap(),
            next: None,
            prev: None,
        })
    }
}

impl Digit {
    pub fn mix(&mut self, digit_count: i64) {
        if self.value == 0 {
            return;
        }

        // detach current
        let my_ref = self
            .next
            .as_ref()
            .unwrap()
            .as_ref()
            .borrow_mut()
            .prev
            .clone()
            .unwrap();
        self.next.as_ref().unwrap().as_ref().borrow_mut().prev = self.prev.clone();
        self.prev.as_ref().unwrap().as_ref().borrow_mut().next = self.next.clone();

        // find target pos
        let steps = self.value % (digit_count - 1);
        let target = if steps < 0 {
            self.traverse(steps)
        } else {
            self.traverse(steps + 1)
        };

        // insert at new position
        self.prev = target.as_ref().borrow_mut().prev.take();
        self.prev.as_ref().unwrap().as_ref().borrow_mut().next = Some(Rc::clone(&my_ref));
        target.as_ref().borrow_mut().prev = Some(my_ref);
        self.next = Some(target);
    }

    pub fn traverse<'a>(&'a self, steps: i64) -> Rc<RefCell<Digit>> {
        if steps == 0 {
            return Rc::clone(
                self.prev
                    .as_ref()
                    .unwrap()
                    .as_ref()
                    .borrow()
                    .next
                    .as_ref()
                    .unwrap(),
            );
        } else if steps < 0 {
            return self
                .prev
                .as_ref()
                .unwrap()
                .as_ref()
                .borrow()
                .traverse(steps + 1);
        } else {
            return self
                .next
                .as_ref()
                .unwrap()
                .as_ref()
                .borrow()
                .traverse(steps - 1);
        }
    }
}

// Convert input list to custom double linked list
fn parse_input(input: String) -> Vec<Rc<RefCell<Digit>>> {
    let dummy_start: Rc<RefCell<Digit>> =
        Rc::new(RefCell::new(Digit::from_str("0").unwrap().into()));
    let mut prev = dummy_start.clone();
    let list: Vec<Rc<RefCell<Digit>>> = input
        .trim()
        .split("\n")
        .into_iter()
        .map(|d| {
            let digit = Digit {
                value: d.parse::<i64>().unwrap(),
                next: None,
                prev: Some(Rc::clone(&prev)),
            };
            let next_prev = Rc::new(RefCell::new(digit));
            prev.as_ref().borrow_mut().next = Some(Rc::clone(&next_prev));
            prev = Rc::clone(&next_prev);
            next_prev
        })
        .collect();
    let first_node = list.first().unwrap();
    let last_node = list.last().unwrap();
    first_node.as_ref().borrow_mut().prev = Some(Rc::clone(last_node));
    last_node.as_ref().borrow_mut().next = Some(Rc::clone(first_node));
    return list;
}

#[async_trait]
impl Solution for Puzzle {
    async fn solve_a(&mut self, input: String) -> Result<Answer, String> {
        let list = parse_input(input);
        let list_len = list.len() as i64;
        for digit in list.iter() {
            digit.as_ref().borrow_mut().mix(list_len);
        }
        let _0th_digit = list.iter().find(|&d| d.borrow().value == 0).unwrap();
        let _1000th_digit = _0th_digit.borrow().traverse(1000);
        let _2000th_digit = _1000th_digit.as_ref().borrow().traverse(1000);
        let _3000th_digit = _2000th_digit.as_ref().borrow().traverse(1000);

        let result = _1000th_digit.borrow().value
            + _2000th_digit.borrow().value
            + _3000th_digit.borrow().value;
        Answer::from(result).into()
    }

    async fn solve_b(&mut self, input: String) -> Result<Answer, String> {
        let list = parse_input(input);
        let list_len = list.len() as i64;
        let decipher_key = 811589153;
        for digit in list.iter() {
            digit.as_ref().borrow_mut().value *= decipher_key;
        }

        for _ in 0..10 {
            for digit in list.iter() {
                digit.as_ref().borrow_mut().mix(list_len);
            }
        }
        let _0th_digit = list.iter().find(|&d| d.borrow().value == 0).unwrap();
        let _1000th_digit = _0th_digit.borrow().traverse(1000);
        let _2000th_digit = _1000th_digit.as_ref().borrow().traverse(1000);
        let _3000th_digit = _2000th_digit.as_ref().borrow().traverse(1000);

        let result = _1000th_digit.borrow().value
            + _2000th_digit.borrow().value
            + _3000th_digit.borrow().value;
        Answer::from(result).into()
    }
}

#[cfg(test)]
mod tests {
    use super::Puzzle;
    use common::{Answer, Solution};

    const TEST_INPUT: &str = "1
2
-3
3
-2
0
4";

    #[tokio::test]
    async fn part_a() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_a(String::from(TEST_INPUT)).await,
            Ok(Answer::from(3))
        )
    }

    #[tokio::test]
    async fn part_b() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_b(String::from(TEST_INPUT)).await,
            Ok(Answer::from(1623178306))
        )
    }
}
