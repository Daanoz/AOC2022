use std::{collections::{HashMap, VecDeque}, fmt::{Debug, Display}};

use async_trait::async_trait;
use common::{Answer, Solution};

pub struct Puzzle {}

#[derive(Clone, PartialEq)]
enum Cell {
    Elf,
    Empty,
}

impl Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Elf => write!(f, "#"),
            Cell::Empty => write!(f, "."),
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Elf => write!(f, "#"),
            Cell::Empty => write!(f, "."),
        }
    }
}

impl Default for Puzzle {
    fn default() -> Self {
        Self {}
    }
}

type Grid = VecDeque<VecDeque<Cell>>;

fn read_cell(grid: &Grid, row: isize, col: isize) -> Cell {
    if let Some(row) = grid.get(row as usize) {
        if let Some(cell) = row.get(col as usize) {
            return cell.clone();
        }
    }
    return Cell::Empty;
}

fn set_cell(grid: &mut Grid, row: isize, col: isize, cell: Cell) {
    grid.get_mut(row as usize).unwrap()[col as usize] = cell;
}

fn make_moves(grid: &Grid, round_index: isize) -> Option<Grid> {
    // start new grid, 1 tile slightly bigger
    let mut next: Grid = grid.iter().map(|row| VecDeque::from(vec![Cell::Empty; row.len() + 2])).collect();
    next.push_back(next.front().unwrap().clone());
    next.push_back(next.front().unwrap().clone());
    let mut move_map: HashMap<isize, (isize, isize, Cell)> = HashMap::new();
    for (row_index, row) in grid.iter().enumerate() {
        for (col_index, cell) in row.iter().enumerate() {
            if cell != &Cell::Elf {
                continue;
            }
            let neighbors = [
                [
                    read_cell(&grid, row_index as isize - 1, col_index as isize - 1),
                    read_cell(&grid, row_index as isize - 1, col_index as isize + 0),
                    read_cell(&grid, row_index as isize - 1, col_index as isize + 1),
                ],
                [
                    read_cell(&grid, row_index as isize + 0, col_index as isize - 1),
                    Cell::Empty,
                    read_cell(&grid, row_index as isize + 0, col_index as isize + 1),
                ],
                [
                    read_cell(&grid, row_index as isize + 1, col_index as isize - 1),
                    read_cell(&grid, row_index as isize + 1, col_index as isize + 0),
                    read_cell(&grid, row_index as isize + 1, col_index as isize + 1),
                ]
            ];
            // due to the slightly bigger grid, we need to offset the index
            let row_index = (row_index + 1) as isize;
            let col_index = (col_index + 1) as isize;
            let has_neighbor = neighbors.iter().any(|c| c.iter().any(|c| c == &Cell::Elf));
            if !has_neighbor {
                set_cell(&mut next, row_index, col_index, cell.clone());
                continue;
            }

            let moves = [
                if neighbors[0].iter().all(|c| c == &Cell::Empty) {
                    Some((row_index - 1, col_index))
                } else {
                    None
                },
                if neighbors[2].iter().all(|c| c == &Cell::Empty) {
                    Some((row_index + 1, col_index))
                } else {
                    None
                },
                if neighbors.iter().all(|c| c[0] == Cell::Empty) {
                    Some((row_index, col_index - 1))
                } else {
                    None
                },
                if neighbors.iter().all(|c| c[2] == Cell::Empty) {
                    Some((row_index, col_index + 1))
                } else {
                    None
                }
            ];   
            let mut next_p = (row_index, col_index);
            for dm in round_index..(round_index + 4) {
                if let Some(valid_move) = moves[dm as usize % 4] {
                    next_p = valid_move;
                    break;
                }
            }  

            let move_key: isize = (next_p.0 * 1000 + next_p.1) as isize;
            let existing_move = move_map.get(&move_key);
            let existing_move = if let Some(existing_move) = existing_move {
                existing_move
            } else {
                set_cell(&mut next, next_p.0, next_p.1, cell.clone());
                move_map.insert(move_key, (row_index, col_index, cell.clone()));
                continue;
            };
            let current_cell = read_cell(&next, next_p.0, next_p.1);
            if current_cell == Cell::Elf {
                // has clashed, reset position
                set_cell(&mut next, next_p.0, next_p.1, Cell::Empty);
                set_cell(&mut next, existing_move.0, existing_move.1, existing_move.2.clone());
            }
            set_cell(&mut next, row_index, col_index, cell.clone());
        }
    }
    if move_map.len() < 1 {
        return None
    }
    Some(trim(next))
}

fn read_grid(input: String) -> Grid {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => Cell::Elf,
                    '.' => Cell::Empty,
                    _ => panic!("Invalid cell"),
                })
                .collect()
        })
        .collect()
}

fn trim(mut grid: Grid) -> Grid {
    while grid.front().unwrap().iter().all(|cell| cell == &Cell::Empty) {
        grid.pop_front();
    }
    while grid.back().unwrap().iter().all(|cell| cell == &Cell::Empty) {
        grid.pop_back();
    }
    while grid.iter().all(|row| row.front().unwrap() == &Cell::Empty) {
        grid.iter_mut().for_each(|row| { row.pop_front(); });
    }
    while grid.iter().all(|row| row.back().unwrap() == &Cell::Empty) {
        grid.iter_mut().for_each(|row| { row.pop_back(); });
    }
    grid
}

#[async_trait]
impl Solution for Puzzle {
    async fn solve_a(&mut self, input: String) -> Result<Answer, String> {
        let mut grid = read_grid(input);
        for i in 0..10 {
            grid = make_moves(&grid, i).unwrap_or(grid);
        }
        Answer::from(grid.iter().fold(0, |sum, row| {
            sum + row.iter().filter(|cell| cell == &&Cell::Empty).count()
        }))
        .into()
    }

    async fn solve_b(&mut self, input: String) -> Result<Answer, String> {
        let mut grid = read_grid(input);
        for i in 0..10000 {
            let next_grid = make_moves(&grid, i);
            if next_grid.is_none() {
                return Answer::from(i + 1).into();
            }
            grid = next_grid.unwrap();
        }
        Answer::from(0).into()
    }
}

#[cfg(test)]
mod tests {
    use super::Puzzle;
    use common::{Answer, Solution};

    const SMALL_TEST_INPUT: &str = ".....
..##.
..#..
.....
..##.
......";

    #[tokio::test]
    async fn part_a_small() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_a(String::from(SMALL_TEST_INPUT)).await,
            Ok(Answer::from(26))
        )
    }

    const TEST_INPUT: &str = "..............
..............
.......#......
.....###.#....
...#...#.#....
....#...##....
...#.###......
...##.#.##....
....#..#......
..............
..............
..............";

    #[tokio::test]
    async fn part_a() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_a(String::from(TEST_INPUT)).await,
            Ok(Answer::from(110))
        )
    }

    #[tokio::test]
    async fn part_b() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_b(String::from(TEST_INPUT)).await,
            Ok(Answer::from(20))
        )
    }
}
