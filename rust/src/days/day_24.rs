use std::{cell::RefCell, collections::HashMap, str::FromStr};

use async_trait::async_trait;
use common::{Answer, Solution};

type Coord = (usize, usize);
type Bounds = (Coord, Coord);

#[derive(Debug, Clone)]
struct Blizzard(Direction, (usize, usize));

#[derive(Debug, Clone)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            ">" => Ok(Direction::Right),
            "v" => Ok(Direction::Down),
            "<" => Ok(Direction::Left),
            "^" => Ok(Direction::Up),
            _ => Err(format!("Invalid direction: {}", s)),
        }
    }
}

pub struct Puzzle {
    blizzards_per_turn: RefCell<HashMap<usize, Vec<Blizzard>>>,
}

impl Default for Puzzle {
    fn default() -> Self {
        Self {
            blizzards_per_turn: RefCell::new(HashMap::new()),
        }
    }
}

impl Puzzle {
    fn get_blizzards_for_turn(&self, turn: usize, bounds: &Bounds) -> Vec<Blizzard> {
        let blizzards_per_turn = self.blizzards_per_turn.borrow().get(&turn).cloned();
        if let Some(blizzards) = blizzards_per_turn {
            return blizzards.clone();
        }
        let blizzards = self
            .get_blizzards_for_turn(turn - 1, bounds)
            .iter()
            .map(|blizzard| {
                let (row, col) = blizzard.1;
                let (row, col) = match blizzard.0 {
                    Direction::Right if col + 1 <= bounds.1.1 => (row, col + 1),
                    Direction::Right => (row, bounds.1.0),
                    Direction::Left if col - 1 >= bounds.1.0 => (row, col - 1),
                    Direction::Left => (row, bounds.1.1),
                    Direction::Down if row + 1 <= bounds.0.1 => (row + 1, col),
                    Direction::Down => (bounds.0.0, col),
                    Direction::Up if row - 1 >= bounds.0.0 => (row - 1, col),
                    Direction::Up => (bounds.0.1, col),
                };
                Blizzard(blizzard.0.clone(), (row, col))
            })
            .collect::<Vec<Blizzard>>();
        self.blizzards_per_turn
            .borrow_mut()
            .insert(turn, blizzards.clone());
        blizzards
    }

    fn run_bfs(&self, offset: usize, bounds: &Bounds, start: &Coord, end: &Coord) -> usize {
        let mut positions: Vec<Coord> = vec![start.clone()];
        
        for i in (1 + offset)..5000 {
            let blizzards: Vec<usize> = self.get_blizzards_for_turn(i, bounds)
                .iter()
                .map(|b| coord_to_hash(b.1))
                .collect();
            let mut next_pos: HashMap<usize, Coord> = HashMap::new();
            for pos in positions {
                let mut moves: Vec<Coord> = vec![pos];
                if pos == *end {
                    return i - 1;
                }
                let is_in_y_bounds = pos.0 >= bounds.0.0 && pos.0 <= bounds.0.1;
                if pos.1 > bounds.1.0 && is_in_y_bounds {
                    moves.push((pos.0, pos.1 - 1))
                }
                if pos.1 < bounds.1.1 && is_in_y_bounds {
                    moves.push((pos.0, pos.1 + 1))
                }
                if pos.0 > bounds.0.0 {
                    moves.push((pos.0 - 1, pos.1))
                } else if pos.0 > 0 {
                    if (pos.0 - 1, pos.1) == *end || (pos.0 - 1, pos.1) == *start {
                        moves.push((pos.0 - 1, pos.1))
                    }
                }
                if pos.0 < bounds.0.1 || (pos.0 + 1, pos.1) == *end || (pos.0 + 1, pos.1) == *start {
                    moves.push((pos.0 + 1, pos.1))
                }
                moves
                    .into_iter()
                    .map(|m| (coord_to_hash(m), m))
                    .for_each(|(hash, mv)| {
                        if !blizzards.contains(&hash) && !next_pos.contains_key(&hash) {
                            next_pos.insert(hash, mv);
                        }
                    })
            }
            positions = next_pos.into_values().collect();
        }
        return 0;
    }
}

fn coord_to_hash(coord: Coord) -> usize {
    coord.0 * 1000 + coord.1
}

fn get_blizzards(input: &str) -> Vec<Blizzard> {
    input
        .lines()
        .enumerate()
        .map(|(row_index, row)| {
            row.chars()
                .enumerate()
                .filter_map(|(col_index, char)| {
                    char.to_string()
                        .parse::<Direction>()
                        .ok()
                        .map(|direction| (col_index, direction))
                })
                .map(|(col_index, direction)| (Blizzard(direction, (row_index, col_index))))
                .collect::<Vec<Blizzard>>()
        })
        .flatten()
        .collect()
}

fn get_bounds(input: &str) -> Bounds {
    (
        (1, input.lines().count() - 2),
        (1, input.lines().next().unwrap().chars().count() - 2),
    )
}

#[async_trait]
impl Solution for Puzzle {
    async fn solve_a(&mut self, input: String) -> Result<Answer, String> {
        let blizzards = get_blizzards(&input);
        self.blizzards_per_turn.borrow_mut().insert(0, blizzards);
        let bounds = get_bounds(&input);
        let start = (bounds.0.0 - 1, bounds.1.0);
        let end = (bounds.0.1 + 1, bounds.1.1);
        Answer::from(self.run_bfs(0, &bounds, &start, &end)).into()
    }

    async fn solve_b(&mut self, input: String) -> Result<Answer, String> {
        let blizzards = get_blizzards(&input);
        self.blizzards_per_turn.borrow_mut().insert(0, blizzards);
        let bounds = get_bounds(&input);
        let start = (bounds.0.0 - 1, bounds.1.0);
        let end = (bounds.0.1 + 1, bounds.1.1);

        let first_pass = self.run_bfs(0, &bounds, &start, &end);
        let second_pass = self.run_bfs(first_pass, &bounds, &end, &start);
        let third_pass = self.run_bfs(second_pass, &bounds, &start, &end);
        Answer::from(third_pass).into()
    }
}

#[cfg(test)]
mod tests {
    use super::Puzzle;
    use common::{Answer, Solution};

    const TEST_INPUT: &str = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";

    #[tokio::test]
    async fn part_a() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_a(String::from(TEST_INPUT)).await,
            Ok(Answer::from(18))
        )
    }

    #[tokio::test]
    async fn part_b() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_b(String::from(TEST_INPUT)).await,
            Ok(Answer::from(54))
        )
    }
}
