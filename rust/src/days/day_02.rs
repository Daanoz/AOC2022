use async_trait::async_trait;
use common::{Answer, Solution};

#[derive(Default)]
pub struct Puzzle {}

#[async_trait]
impl Solution for Puzzle {
    async fn solve_a(&mut self, input: String) -> Result<Answer, String> {
        let scores: Vec<i32> = input
            .trim()
            .split('\n')
            .filter_map(get_round_score)
            .collect();
        Ok(scores.iter().sum::<i32>().into())
    }

    async fn solve_b(&mut self, input: String) -> Result<Answer, String> {
        let scores: Vec<i32> = input
            .trim()
            .split('\n')
            .filter_map(get_round_score_by_result)
            .collect();
        Ok(scores.iter().sum::<i32>().into())
    }
}

const LOSS_SCORE: i32 = 0;
const DRAW_SCORE: i32 = 3;
const WIN_SCORE: i32 = 6;

const ROCK: i32 = 1;
const PAPER: i32 = 2;
const SCISSORS: i32 = 3;

fn get_round_score(round: &str) -> Option<i32> {
    let (opponent_play, response_play) = round.split_once(' ')?;

    let opponent = opponent_play_to_value(opponent_play)?;
    let response = response_play_to_value(response_play)?;

    if response == opponent {
        Some(response + DRAW_SCORE)
    } else if (response == ROCK && opponent == SCISSORS) || (response == (opponent + 1)) {
        Some(response + WIN_SCORE)
    } else {
        Some(response + LOSS_SCORE)
    }
}

fn get_round_score_by_result(round: &str) -> Option<i32> {
    let (opponent_play, result_play) = round.split_once(' ')?;

    let opponent = opponent_play_to_value(opponent_play)?;
    let result = result_to_value(result_play)?;

    match result {
        DRAW_SCORE => Some(opponent + DRAW_SCORE),
        WIN_SCORE => Some(if (opponent + 1) == 4 { 1 } else { opponent + 1 } + WIN_SCORE),
        _ => Some(if (opponent - 1) == 0 { 3 } else { opponent - 1 } + LOSS_SCORE),
    }
}

fn opponent_play_to_value(play: &str) -> Option<i32> {
    match play {
        "A" => Some(ROCK),
        "B" => Some(PAPER),
        "C" => Some(SCISSORS),
        _ => None,
    }
}

fn response_play_to_value(play: &str) -> Option<i32> {
    match play {
        "X" => Some(ROCK),
        "Y" => Some(PAPER),
        "Z" => Some(SCISSORS),
        _ => None,
    }
}

fn result_to_value(play: &str) -> Option<i32> {
    match play {
        "X" => Some(LOSS_SCORE),
        "Y" => Some(DRAW_SCORE),
        "Z" => Some(WIN_SCORE),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::Puzzle;
    use common::{Answer, Solution};

    const TEST_INPUT: &str = "A Y\nB X\nC Z\n";

    #[tokio::test]
    async fn part_a() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_a(String::from(TEST_INPUT)).await,
            Ok(Answer::from(15))
        )
    }

    #[tokio::test]
    async fn part_b() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_b(String::from(TEST_INPUT)).await,
            Ok(Answer::from(12))
        )
    }
}
