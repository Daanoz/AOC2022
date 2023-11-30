use std::collections::HashMap;

use async_trait::async_trait;
use common::{Answer, Solution};

pub struct Puzzle {}

impl Default for Puzzle {
    fn default() -> Self {
        Self { }
    }
}

type Coord = (i32, i32);
type Grid = HashMap<Coord, bool>;

struct Rope {
    head: Coord,
    rope: Vec<Coord>
}

impl Rope {
    pub fn with_length(length: usize) -> Self {
        Self {
            head: (0 ,0),
            rope: vec![(0, 0); length - 1]
        }
    }
    pub fn visit_cell(&self, grid: &mut Grid) {
        let tail = self.rope.last().expect("Rope with no length?");
        grid.insert(tail.to_owned(), true);
    }
    pub fn move_in_direction(&mut self, direction: &str) {
        match direction {
            "U" => self.head = (self.head.0, self.head.1 + 1),
            "D" => self.head = (self.head.0, self.head.1 - 1),
            "L" => self.head = (self.head.0 - 1, self.head.1),
            "R" => self.head = (self.head.0 + 1, self.head.1),
            _ => panic!("Unexpected direction")
        }
        let mut prev = self.head.clone();
        for current in self.rope.iter_mut() {
            if prev == current.clone() {
                prev = current.clone();
                continue;
            }
            let delta_x = if prev.0 < current.0 { -1 } else { 1 };
            let delta_y = if prev.1 < current.1 { -1 } else { 1 };
            if (prev.0 - current.0).abs() >= 2 && (prev.1 - current.1).abs() >= 2 {
                *current = (current.0 + delta_x, current.1 + delta_y);
            } else if (prev.1 - current.1).abs() >= 2 {
                *current = (prev.0, current.1 + delta_y);
            } else if (prev.0 - current.0).abs() >= 2 {
                *current = (current.0 + delta_x, prev.1);
            }
            prev = current.clone();
        }
    }
}

fn move_rope(rope: &mut Rope, grid: &mut Grid, mv: &str) {
    let (direction, distance_str) = mv.split_once(' ').expect("Invalid move command");
    let distance: u32 = distance_str.parse().expect("Invalid move distance");
    for _ in 0..distance {
        rope.move_in_direction(direction);
        rope.visit_cell(grid);
    }
}

#[async_trait]
impl Solution for Puzzle {
    async fn solve_a(&mut self, input: String) -> Result<Answer, String> {
        let mut grid = Grid::new();
        let mut rope = Rope::with_length(2);
        input.trim().split('\n').for_each(|mv| move_rope(&mut rope, &mut grid, mv));
        Ok(Answer::from(grid.len()))
    }

    async fn solve_b(&mut self, input: String) -> Result<Answer, String> {
        let mut grid = Grid::new();
        let mut rope = Rope::with_length(10);
        input.trim().split('\n').for_each(|mv| move_rope(&mut rope, &mut grid, mv));
        Ok(Answer::from(grid.len()))
    }
}

#[cfg(test)]
mod tests {
    use super::Puzzle;
    use common::{Answer, Solution};

    const TEST_INPUT_A: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[tokio::test]
    async fn part_a() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_a(String::from(TEST_INPUT_A)).await,
            Ok(Answer::from(13))
        )
    }

    const TEST_INPUT_B: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[tokio::test]
    async fn part_b() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_b(String::from(TEST_INPUT_B)).await,
            Ok(Answer::from(36))
        )
    }
}