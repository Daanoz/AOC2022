use std::collections::HashSet;

use async_trait::async_trait;
use common::{Answer, Solution};

pub struct Puzzle {}

impl Default for Puzzle {
    fn default() -> Self {
        Self {}
    }
}

#[derive(PartialEq)]
struct Cell {
    start: bool,
    exit: bool,
    height: u32,
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        let bottom_char = 'a' as u32;
        Self {
            start: value == 'S',
            exit: value == 'E',
            height: match value {
                'S' => 0,
                'E' => 25,
                _ => value as u32 - bottom_char,
            },
        }
    }
}

type CellPos = (usize, usize);
struct Grid {
    cells: Vec<Vec<Cell>>,
    start: CellPos,
    exit: CellPos,
    longest_distance: u32,
}

enum PathMode {
    Up,
    Down,
}

fn find_start(cells: &Vec<Vec<Cell>>) -> Option<CellPos> {
    for (r, row) in cells.iter().enumerate() {
        for (c, cell) in row.iter().enumerate() {
            if cell.start {
                return Some((r, c));
            }
        }
    }
    None
}
fn find_exit(cells: &Vec<Vec<Cell>>) -> Option<CellPos> {
    for (r, row) in cells.iter().enumerate() {
        for (c, cell) in row.iter().enumerate() {
            if cell.exit {
                return Some((r, c));
            }
        }
    }
    None
}
impl From<String> for Grid {
    fn from(input: String) -> Self {
        let cells: Vec<Vec<Cell>> = input
            .split("\n")
            .map(|row| row.chars().map(|c| c.into()).collect())
            .collect();
        Self {
            start: find_start(&cells).expect("Should have start"),
            exit: find_exit(&cells).expect("Should have exit"),
            cells,
            longest_distance: u32::MAX,
        }
    }
}
impl Grid {
    fn get_neighbors(&self, pos: &CellPos) -> Vec<CellPos> {
        let mut list: Vec<(usize, usize)> = vec![];
        if self.cells.len() > pos.0 + 1 {
            list.push((pos.0 + 1, pos.1));
        }
        if 1 <= pos.0 {
            list.push((pos.0 - 1, pos.1));
        }
        if self.cells[0].len() > pos.1 + 1 {
            list.push((pos.0, pos.1 + 1));
        }
        if 1 <= pos.1 {
            list.push((pos.0, pos.1 - 1));
        }
        list
    }

    fn get(&self, pos: &CellPos) -> &Cell {
        self.cells.get(pos.0).expect("Row should exist").get(pos.1).expect("Cell should exist")
    }

    fn find_shortest_path(&mut self) -> u32 {
        let exit = self.exit.clone();
        let mut visited = HashSet::from([exit.clone()]);
        self.fill_paths(HashSet::from([exit]), &mut visited, 1, PathMode::Down)
    }

    fn find_path(&mut self) -> u32 {
        let start = self.start.clone();
        let mut visited = HashSet::from([start.clone()]);
        self.fill_paths(HashSet::from([start]), &mut visited, 1, PathMode::Up)
    }

    fn fill_paths(
        &mut self,
        current: HashSet<CellPos>,
        visited: &mut HashSet<CellPos>,
        distance: u32,
        path_mode: PathMode,
    ) -> u32 {
        if distance > self.longest_distance {
            return u32::MAX;
        }
        let mut cells_to_check: HashSet<CellPos> = HashSet::new();
        current.iter().for_each(|pos| {
            let cell = self.get(pos);
            let neighbors = self.get_neighbors(pos);
            let neighbors = neighbors
                .iter()
                .filter(|p| {
                    let n_cell = self.get(p);
                    match path_mode {
                        PathMode::Up => n_cell.height <= cell.height + 1,
                        PathMode::Down => n_cell.height >= cell.height - 1,
                    }
                })
                .filter(|p| !(&visited).contains(&p));
            let newly_visited: Vec<CellPos> = neighbors.cloned().collect();
            visited.extend(newly_visited.clone());
            cells_to_check.extend(newly_visited);
        });
        let has_end = match path_mode {
            PathMode::Up => cells_to_check.contains(&self.exit),
            PathMode::Down => cells_to_check
                .iter()
                .any(|c| self.get(c).height == 0),
        };
        if has_end {
            self.longest_distance = self.longest_distance.min(distance);
            return distance;
        }
        self.fill_paths(cells_to_check, visited, distance + 1, path_mode)
    }
}

#[async_trait]
impl Solution for Puzzle {
    async fn solve_a(&mut self, input: String) -> Result<Answer, String> {
        let mut grid = Grid::from(input);
        Ok(Answer::from(grid.find_path()))
    }

    async fn solve_b(&mut self, input: String) -> Result<Answer, String> {
        let mut grid = Grid::from(input);
        Ok(Answer::from(grid.find_shortest_path()))
    }
}

#[cfg(test)]
mod tests {
    use super::Puzzle;
    use common::{Answer, Solution};

    const TEST_INPUT_A: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
    const TEST_INPUT_B: &str = "Saaabqponm
abbbcryxxl
accccszExk
acccctuvwj
abdddefghi";

    #[tokio::test]
    async fn part_a() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_a(String::from(TEST_INPUT_A)).await,
            Ok(Answer::from(31))
        );
        assert_eq!(
            puzzle.solve_a(String::from(TEST_INPUT_B)).await,
            Ok(Answer::from(33))
        );
    }

    #[tokio::test]
    async fn part_b() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_b(String::from(TEST_INPUT_A)).await,
            Ok(Answer::from(29))
        );
        assert_eq!(
            puzzle.solve_b(String::from(TEST_INPUT_B)).await,
            Ok(Answer::from(30))
        );
    }
}
