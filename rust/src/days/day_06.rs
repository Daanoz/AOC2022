use std::collections::HashSet;

use async_trait::async_trait;
use common::{Answer, Solution};

#[derive(Default)]
pub struct Puzzle {}

fn locate_start_marker(input: String, length: usize) -> Option<usize> {
    input
        .chars()
        .collect::<Vec<char>>()
        .windows(length)
        .position(|slice| HashSet::<&char>::from_iter(slice).len() == length)
        .map(|pos| pos + length)
}

#[async_trait]
impl Solution for Puzzle {
    async fn solve_a(&mut self, input: String) -> Result<Answer, String> {
        locate_start_marker(input, 4)
            .map(|v| v.into())
            .ok_or(String::from("Not found"))
    }

    async fn solve_b(&mut self, input: String) -> Result<Answer, String> {
        locate_start_marker(input, 14)
            .map(|v| v.into())
            .ok_or(String::from("Not found"))
    }
}

#[cfg(test)]
mod tests {
    use super::Puzzle;
    use common::{Answer, Solution};

    #[tokio::test]
    async fn part_a() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle
                .solve_a(String::from("bvwbjplbgvbhsrlpgdmjqwftvncz"))
                .await,
            Ok(Answer::from(5))
        );
        assert_eq!(
            puzzle
                .solve_a(String::from("nppdvjthqldpwncqszvftbrmjlhg"))
                .await,
            Ok(Answer::from(6))
        );
        assert_eq!(
            puzzle
                .solve_a(String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"))
                .await,
            Ok(Answer::from(10))
        );
        assert_eq!(
            puzzle
                .solve_a(String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"))
                .await,
            Ok(Answer::from(11))
        );
    }

    #[tokio::test]
    async fn part_b() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle
                .solve_b(String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb"))
                .await,
            Ok(Answer::from(19))
        );
        assert_eq!(
            puzzle
                .solve_b(String::from("bvwbjplbgvbhsrlpgdmjqwftvncz"))
                .await,
            Ok(Answer::from(23))
        );
        assert_eq!(
            puzzle
                .solve_b(String::from("nppdvjthqldpwncqszvftbrmjlhg"))
                .await,
            Ok(Answer::from(23))
        );
        assert_eq!(
            puzzle
                .solve_b(String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"))
                .await,
            Ok(Answer::from(29))
        );
        assert_eq!(
            puzzle
                .solve_b(String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"))
                .await,
            Ok(Answer::from(26))
        );
    }
}
