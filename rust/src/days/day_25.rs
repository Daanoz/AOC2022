use async_trait::async_trait;
use common::{Answer, Solution};

pub struct Puzzle {}

impl Default for Puzzle {
    fn default() -> Self {
        Self {}
    }
}

fn snafu_to_decimal(val: String) -> i64 {
    let digits = val.len();
    return val.chars().enumerate().fold(0, |total, (i, ds)| {
        let d = if ds == '-' {
            -1
        } else if ds == '=' {
            -2
        } else {
            ds.to_digit(10).unwrap() as i64
        };
        total + ((5 as i64).pow((digits - i) as u32 - 1) * d)
    });
}

fn decimal_to_snafu(val: i64) -> String {
    let digits = ((val as f32).ln() / (5.0_f32).ln()).ceil() as usize;
    let mut out: Vec<String> = vec![];
    let mut remainder = val;
    for i in (0..digits).rev() {
        let lowest_next_digit = snafu_to_decimal(vec!["="; i].join(""));
        let snafu_power = ((remainder - lowest_next_digit) as f32 / (5 as i64).pow(i as u32) as f32).floor() as i64;
        if snafu_power != 0 || !out.is_empty() {
            if snafu_power == -2 {
                out.push("=".into());
            } else if snafu_power == -1 {
                out.push("-".into());
            } else {
                out.push(snafu_power.to_string());
            }
            remainder -= snafu_power * (5 as i64).pow(i as u32)
        }
    }
    out.join("")
}

#[async_trait]
impl Solution for Puzzle {
    async fn solve_a(&mut self, input: String) -> Result<Answer, String> {
        let sum = input
            .lines()
            .map(|v| snafu_to_decimal(v.into()))
            .fold(0, |sum, v| sum + v);
        Answer::from(decimal_to_snafu(sum)).into()
    }

    async fn solve_b(&mut self, _input: String) -> Result<Answer, String> {
        Answer::from("Merry Christmas").into()
    }
}

#[cfg(test)]
mod tests {
    use super::Puzzle;
    use common::{Answer, Solution};

    const TEST_INPUT: &str = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";

    #[tokio::test]
    async fn part_a() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_a(String::from(TEST_INPUT)).await,
            Ok(Answer::from("2=-1=0"))
        )
    }
}
