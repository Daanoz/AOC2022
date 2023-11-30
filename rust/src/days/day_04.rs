use async_trait::async_trait;
use common::{Answer, Solution};

#[derive(Default)]
pub struct Puzzle {}

#[async_trait]
impl Solution for Puzzle {
    async fn solve_a(&mut self, input: String) -> Result<Answer, String> {
        let fully_overlapping_sections = input
            .trim()
            .split('\n')
            .filter_map(row_to_ranges)
            .filter(|(range_a, range_b)| range_a.contains(range_b) || range_b.contains(range_a))
            .count();
        Ok(fully_overlapping_sections.into())
    }

    async fn solve_b(&mut self, input: String) -> Result<Answer, String> {
        let overlapping_sections = input
            .trim()
            .split('\n')
            .filter_map(|row| {
                let ranges = row
                    .split(',')
                    .filter_map(parse_range)
                    .collect::<Vec<(i32, i32)>>();
                match &ranges[..] {
                    &[first, second, ..] => Some((first, second)),
                    _ => None,
                }
            })
            .filter(
                |((start_a, end_a), (start_b, end_b))| {
                    (start_a <= start_b) && (end_a >= start_b) || // left boundary
                    (start_a <= end_b)   && (end_a >= end_b)   || // right boundary
                    (start_a >= start_b) && (end_a <= end_b)   || // 0 in 1
                    (start_a <= start_b) && (end_a >= end_b)
                }, // 1 in 0
            )
            .count();

        Ok(overlapping_sections.into())
    }
}

fn row_to_ranges(row: &str) -> Option<(String, String)> {
    let values: Vec<String> = row
        .split(',')
        .filter_map(parse_range)
        .map(|(start, end)| start..(end + 1))
        .map(|digits| digits.map(|d| d.to_string()).collect::<Vec<String>>())
        .map(|str_digits| format!(",{},", str_digits.join(",")))
        .collect();
    match values {
        val if val.len() == 2 => Some((val[0].to_owned(), val[1].to_owned())),
        _ => None,
    }
}

fn parse_range(range_str: &str) -> Option<(i32, i32)> {
    let values: Option<Vec<i32>> = range_str
        .split('-')
        .map(|val| val.parse::<i32>().ok())
        .collect();
    match values {
        Some(v) if v.len() == 2 => Some((v[0].to_owned(), v[1].to_owned())),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::Puzzle;
    use common::{Answer, Solution};

    const TEST_INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[tokio::test]
    async fn part_a() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_a(String::from(TEST_INPUT)).await,
            Ok(Answer::from(2))
        )
    }

    #[tokio::test]
    async fn part_b() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_b(String::from(TEST_INPUT)).await,
            Ok(Answer::from(4))
        )
    }
}
