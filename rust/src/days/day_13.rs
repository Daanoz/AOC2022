use std::{cmp::Ordering, str::FromStr, string::ParseError};

use async_trait::async_trait;
use common::{Answer, Solution};

pub struct Puzzle {}

impl Default for Puzzle {
    fn default() -> Self {
        Self {}
    }
}

#[derive(PartialEq, Clone, Debug)]
enum Msg {
    Value(u8),
    List(Vec<Msg>)
}

impl FromStr for Msg {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if let Ok(val) = input.parse::<u8>() {
            return Ok(Msg::Value(val));
        }
        let input = &input[1..input.len() - 1];
        let mut out: Vec<Msg> = vec![];
        let mut cur = 0;
        let mut depth = 0;
        for (i, c) in input.chars().enumerate() {
            match c {
                ',' if depth == 0 => {
                    let val = input.get(cur..i).expect("Failed to split str");
                    out.push(Msg::from_str(&val).unwrap());
                    cur = i + 1;
                }
                ']' => depth = depth - 1,
                '[' => depth = depth + 1,
                _ => {}
            };
        }
        let val = input.get(cur..).expect("Failed to split str");
        if val.len() > 0 {
            out.push(Msg::from_str(&val).unwrap());
        }
        return Ok(Msg::List(out));
    }
}

fn is_valid_pair(pair: &(&Msg, &Msg)) -> bool {
    match pair {
        (Msg::Value(left_val), Msg::Value(right_val)) => left_val <= right_val,
        (Msg::Value(left_val), right) => is_valid_pair(&(&Msg::List(vec![Msg::Value(left_val.clone())]), right)),
        (left, Msg::Value(right_val)) => is_valid_pair(&(left, &Msg::List(vec![Msg::Value(right_val.clone())]))),
        (Msg::List(left_val), Msg::List(right_val)) => {
            for (i, left_el) in left_val.iter().enumerate() {
                let right_el = right_val.get(i);
                if right_el.is_none() {
                    return false;
                }
                let right_el = right_el.unwrap();
                if left_el == right_el {
                    continue;
                }
                return is_valid_pair(&(left_el, right_el));
            }
            return true;
        },
    }
}

#[async_trait]
impl Solution for Puzzle {
    async fn solve_a(&mut self, input: String) -> Result<Answer, String> {
        let pairs: Vec<(Msg, Msg)> = input
            .split("\n\n")
            .map(|pair| pair.split_once("\n").unwrap())
            .map(|(a, b)| (Msg::from_str(a).unwrap(), Msg::from_str(b).unwrap()))
            .collect();
        let result = pairs.iter().enumerate().fold(0, |sum, (i, (a, b))| {
            sum + if is_valid_pair(&(&a, &b)) { i + 1 } else { 0 }
        });
        Answer::from(result).into()
    }

    async fn solve_b(&mut self, input: String) -> Result<Answer, String> {
        let input = input.trim().replace("\n\n", "\n");
        let messages: Vec<Msg> = input.split("\n").map(|s| Msg::from_str(s).unwrap()).collect();
        let mut messages_refs: Vec<&Msg> = messages.iter().collect();
        let div_1 = Msg::from_str("[[2]]").unwrap();
        let div_2 = Msg::from_str("[[6]]").unwrap();
        messages_refs.push(&div_1);
        messages_refs.push(&div_2);
        messages_refs.sort_by(|a, b| match is_valid_pair(&(a, b)) {
            true => Ordering::Less,
            false => Ordering::Greater,
        });
        let div_1_index = messages_refs
            .iter()
            .position(|m| m == &&div_1)
            .expect("First division to be found")
            + 1;
        let div_2_index = messages_refs
            .iter()
            .position(|m| m == &&div_2)
            .expect("Second division to be found")
            + 1;
        Answer::from(div_1_index * div_2_index).into()
    }
}

#[cfg(test)]
mod tests {
    use super::Puzzle;
    use common::{Answer, Solution};

    const TEST_INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[tokio::test]
    async fn part_a() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_a(String::from(TEST_INPUT)).await,
            Ok(Answer::from(13))
        )
    }
    
    #[tokio::test]
    async fn part_b() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_b(String::from(TEST_INPUT)).await,
            Ok(Answer::from(140))
        )
    }
}
