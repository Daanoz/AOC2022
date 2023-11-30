use std::{cell::RefCell, collections::HashMap, rc::Rc, str::FromStr};

use async_trait::async_trait;
use common::{Answer, Solution};

pub struct Puzzle {}

impl Default for Puzzle {
    fn default() -> Self {
        Self {}
    }
}

static HUMAN: &str = "humn";
static ROOT: &str = "root";

struct Monkey {
    name: String,
    value: Option<i64>,
    left: Option<Rc<RefCell<Monkey>>>,
    right: Option<Rc<RefCell<Monkey>>>,
    operation: Option<Operation>,
    is_human_in_tree: Option<bool>,
}

impl Monkey {
    pub fn get_value(&self) -> i64 {
        if let Some(value) = self.value {
            value
        } else {
            let left_monkey = self.left.as_ref().unwrap().borrow();
            let right_monkey = self.right.as_ref().unwrap().borrow();
            let left_value = left_monkey.get_value();
            let right_value = right_monkey.get_value();
            match self.operation.as_ref().unwrap() {
                Operation::Add => left_value + right_value,
                Operation::Sub => left_value - right_value,
                Operation::Mul => left_value * right_value,
                Operation::Div => left_value / right_value,
            }
        }
    }

    pub fn find_human_value(&mut self, desired_outcome: i64) -> i64 {
        if self.name == HUMAN {
            return desired_outcome;
        }
        if self.value.is_some() {
            return -1;
        }
        if self.left.as_ref().unwrap().borrow_mut().has_human() {
            let right = self.right.as_ref().unwrap().borrow().get_value();
            if self.name == ROOT {
                self.left
                    .as_ref()
                    .unwrap()
                    .borrow_mut()
                    .find_human_value(right)
            } else {
                let target = match self.operation.as_ref().unwrap() {
                    Operation::Add => desired_outcome - right,
                    Operation::Sub => desired_outcome + right,
                    Operation::Mul => desired_outcome / right,
                    Operation::Div => desired_outcome * right,
                };
                return self
                    .left
                    .as_ref()
                    .unwrap()
                    .borrow_mut()
                    .find_human_value(target);
            }
        } else {
            let left = self.left.as_ref().unwrap().borrow().get_value();
            if self.name == ROOT {
                self.right
                    .as_ref()
                    .unwrap()
                    .borrow_mut()
                    .find_human_value(left)
            } else {
                let target = match self.operation.as_ref().unwrap() {
                    Operation::Add => desired_outcome - left,
                    Operation::Sub => left - desired_outcome,
                    Operation::Mul => desired_outcome / left,
                    Operation::Div => left / desired_outcome,
                };
                self.right
                    .as_ref()
                    .unwrap()
                    .borrow_mut()
                    .find_human_value(target)
            }
        }
    }

    pub fn has_human(&mut self) -> bool {
        if self.name == HUMAN {
            return true;
        }
        if let Some(is_human_in_tree) = self.is_human_in_tree {
            is_human_in_tree
        } else if self.value.is_some() {
            false
        } else {
            let result = self.left.as_ref().unwrap().borrow_mut().has_human()
                || self.right.as_ref().unwrap().borrow_mut().has_human();
            self.is_human_in_tree = Some(result);
            result
        }
    }
}

enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

impl FromStr for Operation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operation::Add),
            "-" => Ok(Operation::Sub),
            "*" => Ok(Operation::Mul),
            "/" => Ok(Operation::Div),
            _ => Err(format!("Invalid operation: {}", s)),
        }
    }
}

fn input_as_monkeys(input: String) -> HashMap<String, Rc<RefCell<Monkey>>> {
    let mut monkeys = HashMap::new();
    for line in input.trim().lines() {
        let mut parts = line.split(": ");
        let name = parts.next().unwrap().to_string();
        monkeys.insert(
            name.clone(),
            Rc::new(RefCell::new(Monkey {
                name,
                is_human_in_tree: None,
                value: None,
                left: None,
                right: None,
                operation: None,
            })),
        );
    }
    for line in input.trim().lines() {
        let mut parts = line.split(": ");
        let name = parts.next().unwrap().to_string();
        let action = parts.next().unwrap().to_string();
        let value = action.parse::<i64>().ok();
        if value.is_some() {
            monkeys.get(&name).unwrap().borrow_mut().value = value;
        } else {
            let mut opertion_parts = action.split(" ");
            let left_monkey_name = opertion_parts.next().unwrap().to_string();
            let operation: Operation = opertion_parts.next().unwrap().to_string().parse().unwrap();
            let right_monkey_name = opertion_parts.next().unwrap().to_string();
            {
                let mut monkey = monkeys.get(&name).unwrap().borrow_mut();
                monkey.left = Some(Rc::clone(monkeys.get(&left_monkey_name).unwrap()));
                monkey.operation = Some(operation);
                monkey.right = Some(Rc::clone(monkeys.get(&right_monkey_name).unwrap()));
            }
        };
    }
    monkeys
}

#[async_trait]
impl Solution for Puzzle {
    async fn solve_a(&mut self, input: String) -> Result<Answer, String> {
        let monkeys = input_as_monkeys(input);
        let root_monkey = monkeys.get(ROOT).unwrap();
        let result = root_monkey.borrow().get_value();
        Answer::from(result).into()
    }

    async fn solve_b(&mut self, input: String) -> Result<Answer, String> {
        let monkeys = input_as_monkeys(input);
        let root_monkey = monkeys.get(ROOT).unwrap();
        let result = root_monkey.borrow_mut().find_human_value(0);
        Answer::from(result).into()
    }
}

#[cfg(test)]
mod tests {
    use super::Puzzle;
    use common::{Answer, Solution};

    const TEST_INPUT: &str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

    #[tokio::test]
    async fn part_a() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_a(String::from(TEST_INPUT)).await,
            Ok(Answer::from(152))
        )
    }

    #[tokio::test]
    async fn part_b() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_b(String::from(TEST_INPUT)).await,
            Ok(Answer::from(301))
        )
    }
}
