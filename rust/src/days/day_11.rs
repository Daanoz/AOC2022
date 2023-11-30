use async_trait::async_trait;
use common::{Answer, Solution};
use regex::Regex;

pub struct Puzzle {}

impl Default for Puzzle {
    fn default() -> Self {
        Self {}
    }
}

struct Monkey {
    starting_items: Vec<i64>,
    operation: Box<dyn Fn(i64) -> i64>,
    divisible_by: i64,
    valid: usize,
    invalid: usize,
    actions: i64,
}

fn regex_first_match<'a>(regex: &'a str, input: &'a str) -> &'a str {
    Regex::new(regex)
        .expect("Valid regex")
        .captures(input)
        .expect("At least one match")
        .get(1)
        .expect("First match to be defined")
        .as_str()
}

fn regex_first_match_as_number<T>(regex: &str, input: &str) -> T
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    regex_first_match(regex, input)
        .parse::<T>()
        .expect("Parsable as i32")
}

fn get_operation(operation: &str, operation_arg: &str) -> Box<dyn Fn(i64) -> i64> {
    if operation_arg == "old" {
        return match operation {
            "+" => Box::new(|v| v + v),
            "*" => Box::new(|v| v * v),
            _ => unreachable!(),
        };
    }
    let operation_arg = operation_arg.parse::<i64>().expect("Parsable as i64");
    return match operation {
        "+" => Box::new(move |v| v + operation_arg),
        "*" => Box::new(move |v| v * operation_arg),
        _ => unreachable!(),
    };
}

impl Monkey {
    pub fn get_divisible_by(&self) -> i64 {
        self.divisible_by
    }
    pub fn get_amount_of_actions(&self) -> i64 {
        self.actions
    }

    pub fn do_turn(&mut self, divide: bool, overflow_mod: i64) -> Vec<(usize, i64)> {
        let items = self.starting_items.drain(..);
        items.into_iter().map(|mut item| {
            item %= overflow_mod;
            let level: i64 = if divide {
                (((self.operation)(item) as f64) / 3.0).floor() as i64
            } else {
                (self.operation)(item)
            };
            self.actions += 1;
            if level % self.divisible_by == 0 {
                (self.valid, level)
            } else {
                (self.invalid, level)
            }
        }).collect()
    }

    pub fn catch(&mut self, level: i64) -> () {
        self.starting_items.push(level);
    }
}

impl From<&str> for Monkey {
    fn from(input: &str) -> Self {
        let starting_items: Vec<i64> = regex_first_match(r"Starting items: ([\d, ]*)", input)
            .split(", ")
            .map(|s| s.parse::<i64>().expect("Parsable as i64"))
            .collect();
        let operation_match = Regex::new(r"Operation: new = old (\*|\+) (.+)")
            .expect("Valid regex")
            .captures(input)
            .expect("At least one match");

        Self {
            starting_items,
            operation: get_operation(
                operation_match
                    .get(1)
                    .expect("First match to be defined")
                    .as_str(),
                operation_match
                    .get(2)
                    .expect("Second match to be defined")
                    .as_str(),
            ),
            divisible_by: regex_first_match_as_number(r"Test: divisible by (\d+)", input),
            valid: regex_first_match_as_number(r"If true: throw to monkey (\d+)", input),
            invalid: regex_first_match_as_number(r"If false: throw to monkey (\d+)", input),
            actions: 0,
        }
    }
}

fn run(count: i32, input: String, divide: bool) -> Answer {
    let mut monkeys: Vec<Monkey> = input.split("\n\n").map(|s| Monkey::from(s)).collect();
    let overflow_modulo = monkeys.iter().fold(1, |modv, monkey| modv * monkey.get_divisible_by());
    for _ in 0..count {
        for i in 0..monkeys.len() {
            let throws = monkeys[i].do_turn(divide, overflow_modulo);
            for (m, level) in throws {
                monkeys[m].catch(level);
            }
        }
    }

    let mut actions: Vec<i64> = monkeys.iter().map(|m| m.get_amount_of_actions()).collect();
    actions.sort();
    actions.reverse();
    Answer::from(actions[0] * actions[1])
}

#[async_trait]
impl Solution for Puzzle {
    async fn solve_a(&mut self, input: String) -> Result<Answer, String> {
        Ok(run(20, input, true))
    }

    async fn solve_b(&mut self, input: String) -> Result<Answer, String> {
        Ok(run(10000, input, false))
    }
}

#[cfg(test)]
mod tests {
    use super::Puzzle;
    use common::{Answer, Solution};

    const TEST_INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[tokio::test]
    async fn part_a() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_a(String::from(TEST_INPUT)).await,
            Ok(Answer::from(10605))
        )
    }

    #[tokio::test]
    async fn part_b() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_b(String::from(TEST_INPUT)).await,
            Ok(Answer::from(2713310158 as i64))
        )
    }
}
