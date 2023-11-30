use async_trait::async_trait;
use common::{Answer, Solution};
use ndarray::{Array2, s};

pub struct Puzzle {}

impl Default for Puzzle {
    fn default() -> Self {
        Self {}
    }
}

type PixelGrid = Array2<Pixel>;

#[derive(Clone, PartialEq)]
enum Pixel {
    Empty,
    Wall,
    Floor,
    SandSrc,
    Sand,
}
impl Default for Pixel {
    fn default() -> Self {
        Self::Empty
    }
}
impl std::fmt::Debug for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, " "),
            Self::Wall => write!(f, "█"),
            Self::Floor => write!(f, "▔"),
            Self::SandSrc => write!(f, "+"),
            Self::Sand => write!(f, "."),
        }
    }
}

#[allow(dead_code)]
fn render(a: &PixelGrid) {
    let row_count = a.nrows();
    for row_index in 0..row_count {
        let row: ndarray::ArrayBase<ndarray::ViewRepr<&Pixel>, ndarray::Dim<[usize; 1]>> = a.row(row_index);
        let row_slice = row.slice(s!(400..600));
        for col_index in 0..row_slice.len() {
            let x = row_slice.get(col_index).unwrap();
            print!("{:?}", x);
        }
        print!("|");
        println!();
    }
}


fn ascending_range<T>(r: std::ops::Range<T>) -> std::ops::Range<T> where T: std::cmp::PartialOrd {
    if r.start < r.end {
        r
    } else {
        r.end..r.start
    }
}

fn load_input(sim: &mut PixelGrid, input: String)  {
    let mut lowest_point: usize = 0;
    input.lines().into_iter().for_each(|line| {
        line
            .split(" -> ")
            .map(|c| c.split_once(",").unwrap())
            .map(|c| (c.1.parse::<usize>().unwrap(), c.0.parse::<usize>().unwrap()))
            .collect::<Vec<(usize, usize)>>()
            .windows(2)
            .for_each(|coords| {
                if let [(start_x, start_y), (end_x, end_y)] = coords {
                    if start_x == end_x {
                        sim.slice_mut(s![*start_x, ascending_range(*start_y..*end_y)]).fill(Pixel::Wall);
                    } else {
                        sim.slice_mut(s![ascending_range(*start_x..*end_x), *start_y]).fill(Pixel::Wall);
                    }
                    sim[[*end_x, *end_y]] = Pixel::Wall;
                    lowest_point = *start_x.max(end_x).max(&lowest_point);
                }
            });
    });
    sim[[0, 500]] = Pixel::SandSrc;
    sim.slice_mut(s!(lowest_point + 2, 0..sim.dim().1)).fill(Pixel::Floor);
}

fn drop_sand(sim: &mut PixelGrid, (x, y): (usize, usize)) -> bool {
    match sim[[y + 1, x]] {
        Pixel::Floor => {
            sim[[y, x]] = Pixel::Sand; 
            false // into the abyss
        },
        Pixel::Empty => drop_sand(sim, (x, y + 1)),
        _ if sim[[y + 1, x - 1]] == Pixel::Empty => drop_sand(sim, (x - 1, y + 1)),
        _ if sim[[y + 1, x + 1]] == Pixel::Empty => drop_sand(sim, (x + 1, y + 1)),
        _ => {
            sim[[y, x]] = Pixel::Sand; 
            true // rested
        }
    }
}

#[async_trait]
impl Solution for Puzzle {
    async fn solve_a(&mut self, input: String) -> Result<Answer, String> {
        let mut sim: PixelGrid = PixelGrid::default((300, 1000));

        load_input(&mut sim, input);
        let mut sand_turns = 0;
        while drop_sand(&mut sim, (500, 0)) {
            sand_turns += 1;
        }
        render(&sim);
        Answer::from(sand_turns).into()
    }

    async fn solve_b(&mut self, input: String) -> Result<Answer, String> {
        let mut sim: PixelGrid = PixelGrid::default((300, 1000));
        load_input(&mut sim, input);
        let mut sand_turns = 0;
        while sim[[0, 500]] != Pixel::Sand {
            drop_sand(&mut sim, (500, 0));
            sand_turns += 1;
        }
        // render(&sim);
        Answer::from(sand_turns).into()
    }
}

#[cfg(test)]
mod tests {
    use super::Puzzle;
    use common::{Answer, Solution};

    const TEST_INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[tokio::test]
    async fn part_a() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_a(String::from(TEST_INPUT)).await,
            Ok(Answer::from(""))
        )
    }

    #[tokio::test]
    async fn part_b() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_b(String::from(TEST_INPUT)).await,
            Ok(Answer::from(93))
        )
    }
}
