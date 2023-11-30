use std::collections::HashMap;

use async_trait::async_trait;
use common::{Answer, Solution};

const WIDTH: usize = 7;
type Row = [bool; WIDTH];

type Position = (usize, usize);


type BlockShape = Vec<Vec<bool>>;
type Block = (usize, BlockShape);

struct Minus { }
impl Minus {
    fn get() -> Block {
        (
            4,
            vec![
                vec![true, true, true, true],
            ]
        )
    }
}
struct Plus { }
impl Plus {
    fn get() -> Block {
        (
            3,
            vec![
                vec![false, true, false],
                vec![true, true, true],
                vec![false, true, false],
            ]
        )
    }
}
struct Corner { }
impl Corner {
    fn get() -> Block {
        (
            3,
            vec![
                vec![true, true, true],
                vec![false, false, true],
                vec![false, false, true],
            ]
        )
    }
}
struct Pipe { }
impl Pipe {
    fn get() -> Block {
        (
            1,
            vec![
                vec![true],
                vec![true],
                vec![true],
                vec![true],
            ]
        )
    }
}
struct Square { }
impl Square {
    fn get() -> Block {
        (
            2,
            vec![
                vec![true, true],
                vec![true, true],
            ]
        )
    }
}

#[derive(Debug)]
enum Jet {
    Left,
    Right
}


#[derive(Default)]
pub struct Puzzle {}

pub struct TetrisPuzzle {
    rows: Vec<Row>,
    blocks: Vec<Block>,
    jets: Vec<Jet>,
    jet_index: usize,
    cycle_cache: HashMap<usize, (usize, usize)>
}

impl TetrisPuzzle {
    pub fn new(input: String) -> Self {
        Self { 
            rows: Default::default(),
            blocks: vec![Minus::get(), Plus::get(), Corner::get(), Pipe::get(), Square::get()],
            jets: input_to_jets(input),
            jet_index: 0,
            cycle_cache: Default::default()
        }
    }

    fn get_cell(&self, x: usize, y: usize) -> Option<&bool> {
        let row = self.rows.get(y)?;
        row.get(x)
    }
    fn activate_cell(&mut self, x: usize, y: usize) {
        while self.rows.len() <= y  {
            self.rows.push([false; WIDTH]);
        }
        let row = self.rows.get_mut(y).expect("Row not found");
        row[x] = true;
    }

    fn drop_blocks(&mut self, indexes: std::ops::Range<usize>) -> usize {
        let end = indexes.end;
        for i in indexes {
            let cycle_id = self.cycle_id_for_index(i);
            if i < 2500 || !self.cycle_cache.contains_key(&cycle_id) {
                self.drop_block(i);
                continue;
            }
            // Possible cycle detected
            let (previous_index, bottom) = self.cycle_cache.get(&cycle_id).expect("Cycle should be present");
            let cycle_length: usize = i - previous_index;
            if i % cycle_length != end % cycle_length {
                // Can't calculate the end of the cycle, so just drop the block
                self.drop_block(i);
                continue;
            }
            let current_bottom = self.rows.len();
            let cycle_height = current_bottom - bottom;
            let cycle_count = (end - i) as f64 / cycle_length as f64;
            return (current_bottom as f64 + cycle_height as f64 * cycle_count) as usize;
        }
        self.rows.len()
    }

    fn drop_block(&mut self, index: usize) {
        let cycle_id = self.cycle_id_for_index(index);
        let block = self.blocks.get(index % self.blocks.len()).unwrap().clone();
        let bottom = self.rows.len();
        self.cycle_cache.insert(cycle_id, (index, bottom));

        let mut position: Position = (2, bottom + 3);
        loop {
            let jet = &self.jets[self.jet_index];
            self.jet_index = (self.jet_index + 1) % self.jets.len();
            position = self.apply_jet(position, jet, &block);
            if !self.can_go_down(position, &block) {
                break;
            }
            position = (position.0, position.1 - 1);
        }
        self.write_block_to_rows(position, block);
    }

    fn write_block_to_rows(&mut self, pos: Position, block: Block) {
        let shape = block.1;
        for (y, row) in shape.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if *cell {
                    self.activate_cell(pos.0 + x, pos.1 + y);
                }
            }
        }
    }

    fn apply_jet(&self, pos: Position, jet: &Jet, block: &Block) -> Position {
        match jet {
            Jet::Left if pos.0 >= 1 && self.can_go_left(pos, block) => (pos.0 - 1, pos.1),
            Jet::Right if pos.0 + block.0 < WIDTH && self.can_go_right(pos, block) => (pos.0 + 1, pos.1),
            _ => pos
        }
    }

    fn can_go_left(&self, pos: Position, block: &Block) -> bool {
        for (y, row) in block.1.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if *cell {
                    if self.get_cell(
                        (pos.0 - 1) + x, 
                        pos.1 + y
                    ) == Some(&true) {
                        return false;
                    }
                    break;
                }
            }
        }
        return true;
    }
    
    fn can_go_right(&self, pos: Position, block: &Block) -> bool {
        for (y, row) in block.1.iter().enumerate() {
            for (x, cell) in row.iter().enumerate().rev() {
                if *cell {
                    if self.get_cell((pos.0 + 1) + x, pos.1 + y) == Some(&true) {
                        return false;
                    }
                    break;
                }
            }
        }
        return true;
    }
    
    fn can_go_down(&self, pos: Position, block: &Block) -> bool {
        if pos.1 <= 0 {
            return false;
        }
        for x in 0..block.0 {
            for (y, row) in block.1.iter().enumerate() {
                if row[x] {
                    if self.get_cell(pos.0 + x, (pos.1 - 1) + y) == Some(&true) {
                        return false;
                    }
                    break;
                }
            }
        }
        return true;
    }

    fn cycle_id_for_index(&self, index: usize) -> usize {
        (self.jet_index << self.blocks.len()) + (index % self.blocks.len())
    }
}

impl std::fmt::Display for TetrisPuzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut row_strs = self.rows.iter().map(|r| r.map(|c| if c { '#' } else { '.' }).iter().collect::<String>()).collect::<Vec<_>>();
        row_strs.reverse();
        f.write_str(&row_strs.join("\n"))
    }
}

fn input_to_jets(input: String) -> Vec<Jet> {
    input.chars().map(|c| match c {
        '<' => Jet::Left,
        '>' => Jet::Right,
        _ => panic!("Invalid jet: {}", c),
    }).collect::<Vec<_>>()
}

#[async_trait]
impl Solution for Puzzle {
    async fn solve_a(&mut self, input: String) -> Result<Answer, String> {
        let mut tetris: TetrisPuzzle = TetrisPuzzle::new(input);
        Answer::from(tetris.drop_blocks(0..2022)).into()
    }

    async fn solve_b(&mut self, input: String) -> Result<Answer, String> {
        let mut tetris: TetrisPuzzle = TetrisPuzzle::new(input);
        Answer::from(tetris.drop_blocks(0..1000000000000)).into()
    }
}

#[cfg(test)]
mod tests {
    use super::Puzzle;
    use common::{Answer, Solution};

    const TEST_INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[tokio::test]
    async fn part_a() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_a(String::from(TEST_INPUT)).await,
            Ok(Answer::from(3068))
        )
    }

    #[tokio::test]
    async fn part_b() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_b(String::from(TEST_INPUT)).await,
            Ok(Answer::from(1514285714288 as i64))
        )
    }
}