use async_trait::async_trait;
use common::{Answer, Solution};

#[derive(Default)]
pub struct Puzzle {}

#[async_trait]
impl Solution for Puzzle {
    async fn solve_a(&mut self, input: String) -> Result<Answer, String> {
        let amount_per_elf: Vec<i32> = input_to_amount_per_elf(input);
        Ok(amount_per_elf[0].into())
    }

    async fn solve_b(&mut self, input: String) -> Result<Answer, String> {
        let amount_per_elf = input_to_amount_per_elf(input);
        let top3 = &amount_per_elf[0..3];
        Ok(top3.iter().sum::<i32>().into())
    }
}

fn input_to_amount_per_elf(input: String) -> Vec<i32> {
    let elves: std::str::Split<&str> = input.split("\n\n");
    let mut amount_per_elf: Vec<i32> = elves
        .map(|e| {
            e.trim()
                .split('\n')
                .filter_map(|v| v.parse::<i32>().ok())
                .sum()
        })
        .collect();
    amount_per_elf.sort();
    amount_per_elf.reverse();
    amount_per_elf
}

#[cfg(test)]
mod tests {
    use super::Puzzle;
    use common::{Answer, Solution};

    const TEST_INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[tokio::test]
    async fn part_a() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_a(String::from(TEST_INPUT)).await,
            Ok(Answer::from(24000))
        )
    }

    #[tokio::test]
    async fn part_b() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_b(String::from(TEST_INPUT)).await,
            Ok(Answer::from(45000))
        )
    }
}
