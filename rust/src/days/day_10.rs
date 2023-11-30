use async_trait::async_trait;
use common::{Answer, Solution};

pub struct Puzzle {}

impl Default for Puzzle {
    fn default() -> Self {
        Self { }
    }
}

static SCREEN_WIDTH: i32 = 40;

fn run_program(input: String) -> Vec<i32> {
    let mut history = vec![1];

    input.lines().for_each(|line| {
        let mut parts: Vec<&str> = line.split(" ").collect();
        let command = parts.remove(0);
        history.push(*history.last().unwrap());
        match command {
            "noop" => return,
            "addx" => {
                history.push(
                    history.last().unwrap() + parts[0].parse::<i32>().expect("invalid integer")
                );
            },
            _ => panic!("Unknown command: {}", command)
        }
    });
    history
}

#[async_trait]
impl Solution for Puzzle {
    async fn solve_a(&mut self, input: String) -> Result<Answer, String> {
        let strength: i32 = run_program(input)
            .iter()
            .enumerate()
            .filter(|(index, _)| (((*index as i32) + 1) % SCREEN_WIDTH) == (SCREEN_WIDTH / 2))
            .map(|(index, value)| ((index as i32) + 1) * value)
            .sum();
        Ok(Answer::from(strength))
    }

    async fn solve_b(&mut self, input: String) -> Result<Answer, String> {
        let output: String = run_program(input)
            .iter()
            .enumerate()
            .map(|(index, value)| (((index as i32) % SCREEN_WIDTH) - value).abs() < 2)
            .map(|value| if value { "#" } else { "." })
            .collect::<Vec<_>>()
            .chunks(SCREEN_WIDTH as usize)
            .map(|line| line.join(""))
            .collect::<Vec<_>>()[0..6]
            .join("\n");
        Ok(Answer::from(format!("\n{}", output)))
    }
}

#[cfg(test)]
mod tests {
    use super::Puzzle;
    use common::{Answer, Solution};

    const TEST_INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[tokio::test]
    async fn part_a() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_a(String::from(TEST_INPUT)).await,
            Ok(Answer::from(13140))
        )
    }

    #[tokio::test]
    async fn part_b() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_b(String::from(TEST_INPUT)).await,
            Ok(Answer::from("
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."))
        )
    }
}