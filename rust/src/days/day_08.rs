use async_trait::async_trait;

use common::{Answer, Solution};

#[derive(Default)]
pub struct Puzzle {}

fn set_visibility_for_range<R>(row: &[u32], visibility_row: &mut [bool], range: R, max_height: &u32)
where
    R: DoubleEndedIterator<Item = usize>,
{
    let mut height: i64 = -1;
    for col_index in range {
        if height < row[col_index].into() {
            visibility_row[col_index] = true;
            height = row[col_index].into();
        }
        if i64::from(*max_height) == height {
            break;
        }
    }
}
fn set_visibility_for_row(row: &[u32], visibility_row: &mut [bool]) {
    let max_height = row.iter().max().unwrap_or(&0);
    set_visibility_for_range(row, visibility_row, 0..row.len(), max_height);
    set_visibility_for_range(row, visibility_row, (0..row.len()).rev(), max_height);
}

fn transpose<T>(value: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    if value.is_empty() {
        return vec![];
    }
    let row_count = value[0].len();
    let mut output: Vec<Vec<T>> = vec![];
    for col_index in 0..row_count {
        let mut row = vec![];
        for input_row in value {
            row.push(input_row[col_index].to_owned());
        }
        output.push(row);
    }
    output
}

fn get_score(grid: &[Vec<u32>], row_index: usize, col_index: usize, height: u32) -> usize {
    let row = &grid[row_index];
    let column: Vec<u32> = grid.iter().map(|r| r[col_index]).collect();
    let (left, right) = row.split_at(col_index);
    let (top, bottom) = column.split_at(row_index);
    fn find_sight_line_incrementing(input: &[u32], height: u32) -> usize {
        Vec::from(input)
            .iter()
            .skip(1)
            .position(|h| h >= &height)
            .map(|v| v + 1)
            .unwrap_or(input.len() - 1)
    }
    fn find_sight_line_decrementing(input: &[u32], height: u32) -> usize {
        Vec::from(input)
            .iter()
            .rev()
            .position(|h| h >= &height)
            .map(|v| v + 1)
            .unwrap_or(input.len())
    }
    find_sight_line_incrementing(right, height)
        * find_sight_line_incrementing(bottom, height)
        * find_sight_line_decrementing(left, height)
        * find_sight_line_decrementing(top, height)
}

#[async_trait]
impl Solution for Puzzle {
    async fn solve_a(&mut self, input: String) -> Result<Answer, String> {
        let grid: Vec<Vec<u32>> = input
            .trim()
            .split('\n')
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).expect("Invalid cell found"))
                    .collect()
            })
            .collect();
        let mut visible: Vec<Vec<bool>> = grid
            .iter()
            .map(|row| row.iter().map(|_| false).collect())
            .collect();
        grid.iter()
            .enumerate()
            .for_each(|(index, row)| set_visibility_for_row(row, &mut visible[index]));
        let grid = transpose(&grid);
        let mut visible = transpose(&visible);
        grid.iter()
            .enumerate()
            .for_each(|(index, row)| set_visibility_for_row(row, &mut visible[index]));
        Ok(Answer::from(
            visible
                .iter()
                .flatten()
                .fold(0, |acc, cell| acc + if *cell { 1 } else { 0 }),
        ))
    }

    async fn solve_b(&mut self, input: String) -> Result<Answer, String> {
        let grid: Vec<Vec<u32>> = input
            .trim()
            .split('\n')
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).expect("Invalid cell found"))
                    .collect()
            })
            .collect();
        let high_score = grid
            .iter()
            .enumerate()
            .map(|(row_index, row)| {
                row.iter()
                    .enumerate()
                    .map(|(column_index, height)| {
                        get_score(&grid, row_index, column_index, *height)
                    })
                    .max()
                    .unwrap_or(0)
            })
            .max();
        Ok(Answer::from(high_score.expect("High score")))
    }
}

#[cfg(test)]
mod tests {
    use super::Puzzle;
    use common::{Answer, Solution};

    const TEST_INPUT: &str = "30373
25512
65332
33549
35390";

    #[tokio::test]
    async fn part_a() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_a(String::from(TEST_INPUT)).await,
            Ok(Answer::from(21))
        )
    }

    #[tokio::test]
    async fn part_b() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_b(String::from(TEST_INPUT)).await,
            Ok(Answer::from(8))
        )
    }
}
