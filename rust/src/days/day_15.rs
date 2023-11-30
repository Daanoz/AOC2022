use std::collections::HashSet;

use async_trait::async_trait;
use common::{Answer, Solution};
use regex::Regex;

const TUNING_FREQUENCY_MULTIPLIER: i64 = 4_000_000;
pub struct Puzzle {
    y_row: i64,
    scan_range: i64,
}

impl Default for Puzzle {
    fn default() -> Self {
        Self {
            y_row: 2000000,
            scan_range: TUNING_FREQUENCY_MULTIPLIER,
        }
    }
}

struct Sensor {
    position: (i64, i64),
    beacon: (i64, i64),
    distance: i64,
}

fn parse_input(input: String) -> Vec<Sensor> {
    let regex =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .expect("Valid regex");
    input
        .split('\n')
        .into_iter()
        .map(|line| {
            let captures = regex.captures(line).expect(&format!("At least one match: {}", line));
            let captures = captures
                .iter()
                .skip(1)
                .map(|c| {
                    let str = c.expect("Match").as_str();
                    str.parse().expect(&format!("Invalid integer: {}", str))
                })
                .collect::<Vec<i64>>();
            let (x1, y1, x2, y2) = (captures[0], captures[1], captures[2], captures[3]);
            let distance = (x1 - x2).abs() + (y1 - y2).abs();
            Sensor {
                position: (x1, y1),
                beacon: (x2, y2),
                distance,
            }
        })
        .collect()
}

fn find_horizontal_bounds(coords: &[Sensor]) -> (i64, i64) {
    coords.iter().fold((i64::MAX, i64::MIN), |acc, coord| {
        (
            (coord.position.0 - coord.distance).min(acc.0),
            (coord.position.1 + coord.distance).max(acc.1),
        )
    })
}

fn is_in_range_of_sensor(coords: &[Sensor], (x, y): (i64, i64)) -> bool {
    coords
        .iter()
        .find(|c| ((c.position.0 - x).abs() + (c.position.1 - y).abs()) <= c.distance)
        .is_some()
}

fn find_empty_spots_on_row(coords: &[Sensor], y_row: i64) -> i64 {
    let horizontal_bounds = find_horizontal_bounds(coords);
    let beacons_in_row = coords
        .iter()
        .filter(|c| c.beacon.1 == y_row)
        .fold(HashSet::<i64>::new(), |mut positions, c| {
            positions.insert(c.beacon.0);
            positions
        })
        .len();

    let mut empty = 0;
    for x in horizontal_bounds.0..=horizontal_bounds.1 {
        if is_in_range_of_sensor(coords, (x, y_row)) {
            empty = empty + 1;
        }
    }
    return empty - beacons_in_row as i64;
}

fn find_tuning_frequency(coords: &[Sensor], limit: i64, tuning_frequency_multiplier: i64) -> i64 {
    for coord in coords {
        let delta = coord.distance + 1;
         for d in -delta..=delta {
            let y = coord.position.1 + d;
            let dx = delta - d.abs();
            let left = (coord.position.0 - dx, y);
            let right = (coord.position.0 + dx, y);
            if is_in_bounds(left, (0, limit)) {
                if !is_in_range_of_sensor(coords, left) {
                    return (left.0 * tuning_frequency_multiplier) + y;
                }
            }
            if is_in_bounds(right, (0, limit)) && left.0 != right.0 {
                if !is_in_range_of_sensor(coords, right) {
                    return (right.0 * tuning_frequency_multiplier) + y;
                }
            }
        }
    }
    0
}

fn is_in_bounds(coord: (i64, i64), bounds: (i64, i64)) -> bool {
    coord.0 >= bounds.0 && coord.0 <= bounds.1 && 
    coord.1 >= bounds.0 && coord.1 <= bounds.1
}

#[async_trait]
impl Solution for Puzzle {
    async fn solve_a(&mut self, input: String) -> Result<Answer, String> {
        let coords = parse_input(input);
        Answer::from(find_empty_spots_on_row(&coords, self.y_row)).into()
    }

    async fn solve_b(&mut self, input: String) -> Result<Answer, String> {
        let coords = parse_input(input);
        Answer::from(find_tuning_frequency(
            &coords,
            self.scan_range,
            TUNING_FREQUENCY_MULTIPLIER,
        ))
        .into()
    }
}

#[cfg(test)]
mod tests {
    use super::Puzzle;
    use common::{Answer, Solution};

    const TEST_INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[tokio::test]
    async fn part_a() {
        let mut puzzle = Puzzle {
            y_row: 10,
            scan_range: 20,
        };
        assert_eq!(
            puzzle.solve_a(String::from(TEST_INPUT)).await,
            Ok(Answer::from(26))
        )
    }

    #[tokio::test]
    async fn part_b() {
        let mut puzzle = Puzzle {
            y_row: 10,
            scan_range: 20,
        };
        assert_eq!(
            puzzle.solve_b(String::from(TEST_INPUT)).await,
            Ok(Answer::from(56_000_011))
        )
    }
}
