use std::collections::{HashMap, HashSet, VecDeque};

use async_trait::async_trait;
use common::{Answer, Solution};

type Int = isize;
type Coord = (Int, Int, Int);

pub struct Puzzle {
    map: HashMap<Int, HashMap<Int, HashSet<Int>>>,
}

impl Default for Puzzle {
    fn default() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
}

impl Puzzle {
    fn add_coords(&mut self, coords: &[Coord]) {
        for coord in coords {
            self.map
                .entry(coord.0)
                .or_default()
                .entry(coord.1)
                .or_default()
                .insert(coord.2);
        }
    }

    fn contains_block(&self, c: &Coord) -> bool {
        self.map
            .get(&c.0)
            .and_then(|m| m.get(&c.1))
            .and_then(|s| s.get(&c.2))
            .is_some()
    }

    fn locate_sides(&self, coords: &[Coord]) -> Vec<Coord> {
        let mut sides = vec![];
        for coord in coords {
            let mut neighbors: Vec<Coord> = get_neighbors_of(&coord)
                .into_iter()
                .filter(|n| !self.contains_block(n))
                .collect();
            sides.append(&mut neighbors);
        }
        sides
    }

    fn is_outside(&self, c: &Coord, x_range: &(Int, Int)) -> bool {
        if c.0 < x_range.0 {
            return true;
        }
        if c.0 > x_range.1 {
            return true;
        }
        if let Some(dimension_data) = self.map.get(&c.0) {
            if &c.1 < dimension_data.keys().min().expect("min_value") {
                return true;
            }
            if &c.1 > dimension_data.keys().max().expect("max_value") {
                return true;
            }
            if let Some(dimension_data) = dimension_data.get(&c.1) {
                if &c.2 < dimension_data.iter().min().expect("min_value") {
                    return true;
                }
                if &c.2 > dimension_data.iter().max().expect("max_value") {
                    return true;
                }
            }
        }
        return false;
    }

    fn find_closed_sides(&self, sides: Vec<Coord>, x_range: &(Int, Int)) -> Vec<Coord> {
        let mut closed_coords: Vec<Coord> = vec![];
        let mut open_coords: Vec<Coord> = vec![];
        let closed_sides: Vec<Coord> = sides
            .into_iter()
            .filter(|c| {
                if closed_coords.contains(c) {
                    return true;
                }
                let mut visited = vec![*c];
                let mut queue: VecDeque<Coord> = get_neighbors_of(c)
                    .into_iter()
                    .filter(|c| !self.contains_block(c))
                    .collect();
                while let Some(current) = queue.pop_front() {
                    if visited.contains(&&current) {
                        continue;
                    }
                    if open_coords.contains(&current) {
                        open_coords.push(current);
                        return false;
                    }
                    visited.push(current);
                    if self.is_outside(&current, x_range) {
                        open_coords.push(current);
                        return false;
                    }
                    let mut next = get_neighbors_of(&current)
                        .into_iter()
                        .filter(|n| !self.contains_block(n))
                        .filter(|n| !visited.contains(n))
                        .collect::<VecDeque<Coord>>();
                    queue.append(&mut next);
                }
                closed_coords.append(&mut visited);
                return true;
            })
            .collect();
        closed_sides
    }
}

fn get_neighbors_of(c: &Coord) -> Vec<Coord> {
    let deltas: [isize; 2] = [-1, 1];
    let mut neighbors = vec![];
    for d in deltas {
        neighbors.push((c.0 + d, c.1, c.2));
        neighbors.push((c.0, c.1 + d, c.2));
        neighbors.push((c.0, c.1, c.2 + d));
    }
    return neighbors;
}

fn coords_from_input(str: &str) -> Vec<Coord> {
    str.trim()
        .split('\n')
        .map(|s| coord_from_str(s))
        .collect::<Vec<_>>()
}

fn coord_from_str(str: &str) -> Coord {
    let values = str
        .split(',')
        .map(|s| s.parse::<Int>().expect("Valid usize"))
        .collect::<Vec<_>>();
    (values[0], values[1], values[2])
}

#[async_trait]
impl Solution for Puzzle {
    async fn solve_a(&mut self, input: String) -> Result<Answer, String> {
        let coords = coords_from_input(&input);
        self.add_coords(&coords);
        Answer::from(self.locate_sides(&coords).len()).into()
    }

    async fn solve_b(&mut self, input: String) -> Result<Answer, String> {
        let coords = coords_from_input(&input);
        self.add_coords(&coords);
        let x_range = coords
            .iter()
            .map(|c| c.0)
            .fold((Int::MAX, Int::MIN), |range, c| {
                (range.0.min(c), range.1.max(c))
            });
        let sides = self.locate_sides(&coords);

        let mut inside_sides = sides.clone();
        inside_sides.sort();
        inside_sides.dedup();
        let inside_sides: Vec<Coord> = inside_sides
            .into_iter()
            .filter(|c| !self.is_outside(&c, &x_range))
            .collect();

        let closed_sides = self.find_closed_sides(inside_sides, &x_range);
        Answer::from(
            sides
                .into_iter()
                .filter(|s| !closed_sides.contains(s))
                .count(),
        )
        .into()
    }
}

#[cfg(test)]
mod tests {
    use super::Puzzle;
    use common::{Answer, Solution};

    const TEST_INPUT: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    #[tokio::test]
    async fn part_a() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_a(String::from(TEST_INPUT)).await,
            Ok(Answer::from(64))
        )
    }

    #[tokio::test]
    async fn part_b() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_b(String::from(TEST_INPUT)).await,
            Ok(Answer::from(58))
        )
    }
}
