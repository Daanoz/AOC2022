use async_trait::async_trait;
use common::{Answer, Solution};
use regex::Regex;

pub struct Puzzle {
    move_regex: Regex,
}

impl Default for Puzzle {
    fn default() -> Self {
        Self {
            move_regex: Regex::new(r"^move (\d+) from (\d+) to (\d+)$")
                .expect("Failed to construct regex"),
        }
    }
}

#[async_trait]
impl Solution for Puzzle {
    async fn solve_a(&mut self, input: String) -> Result<Answer, String> {
        let (stacks_str, moves) = input
            .split_once("\n\n")
            .ok_or("Input not matching expected format")?;
        let mut stacks = parse_stacks(stacks_str).ok_or("Expected to parse into stacks")?;
        moves
            .split('\n')
            .try_for_each(|move_str| self.make_move(&mut stacks, move_str, false))
            .map_err(|e| e.to_string())?;
        let top_stacks: Vec<String> = stacks
            .into_iter()
            .map(|stack| stack[stack.len() - 1].to_string())
            .collect();

        Ok(top_stacks.join("").into())
    }

    async fn solve_b(&mut self, input: String) -> Result<Answer, String> {
        let (stacks_str, moves) = input
            .split_once("\n\n")
            .ok_or("Input not matching expected format")?;
        let mut stacks = parse_stacks(stacks_str).ok_or("Expected to parse into stacks")?;
        moves
            .split('\n')
            .try_for_each(|move_str| self.make_move(&mut stacks, move_str, true))
            .map_err(|e| e.to_string())?;
        let top_stacks: Vec<String> = stacks
            .into_iter()
            .map(|stack| stack[stack.len() - 1].to_string())
            .collect();

        Ok(top_stacks.join("").into())
    }
}

impl Puzzle {
    fn make_move(
        &self,
        stacks: &mut [Vec<char>],
        move_str: &str,
        is_cratemover_9001: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let captures = self
            .move_regex
            .captures(move_str)
            .ok_or(format!("Failed to match regex: {}", move_str))?;
        let count: usize = captures.get(1).map_or("", |m| m.as_str()).parse()?;
        let src: usize = captures.get(2).map_or("0", |m| m.as_str()).parse()?;
        let target: usize = captures.get(3).map_or("0", |m| m.as_str()).parse()?;
        if is_cratemover_9001 {
            let stack_length = stacks[src - 1].len();
            let values: Vec<_> = stacks[src - 1].drain((stack_length - count)..).collect();
            stacks[target - 1].extend(values.iter());
        } else {
            for _ in 0..count {
                let value = stacks[src - 1].pop().unwrap();
                stacks[target - 1].push(value);
            }
        }
        Ok(())
    }
}

fn parse_stacks(stacks_str: &str) -> Option<Vec<Vec<char>>> {
    let mut stacks: Vec<Vec<char>> = Vec::new();
    let mut rows: Vec<&str> = stacks_str.split('\n').collect();

    let column_header = rows.pop()?;

    for _char in column_header.chars().skip(1).step_by(4) {
        stacks.push(Vec::new());
    }
    rows.into_iter().rev().for_each(|row| {
        row.chars()
            .skip(1)
            .step_by(4)
            .enumerate()
            .for_each(|(i, c)| {
                if !c.is_whitespace() {
                    stacks[i].push(c);
                }
            });
    });

    Some(stacks)
}

#[cfg(test)]
mod tests {
    use super::Puzzle;
    use common::{Answer, Solution};

    const TEST_INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[tokio::test]
    async fn part_a() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_a(String::from(TEST_INPUT)).await,
            Ok(Answer::from("CMZ"))
        )
    }

    #[tokio::test]
    async fn part_b() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_b(String::from(TEST_INPUT)).await,
            Ok(Answer::from("MCD"))
        )
    }
}
