use async_trait::async_trait;
use common::{Answer, Solution};

#[derive(Default)]
pub struct Puzzle {}

#[async_trait]
impl Solution for Puzzle {
    async fn solve_a(&mut self, input: String) -> Result<Answer, String> {
        let bags = input.trim().split('\n');
        let bags_with_compartments = bags
            .to_owned()
            .map(|r| r.split_at(r.len() / 2))
            .map(|bag| (bag.0.to_owned(), bag.1.to_owned()));

        let priority_sum: Option<u32> = bags_with_compartments
            .map(|bag| {
                bag.0
                    .chars()
                    .filter(|c| bag.1.chars().any(|c2| c2.cmp(c).is_eq()))
                    .collect::<Vec<char>>()
            })
            .flat_map(|mut chars| {
                chars.sort();
                chars.dedup();
                chars
            })
            .map(|c| get_item_priority(&Some(c)))
            .sum();

        Ok(priority_sum.unwrap_or(0).into())
    }

    async fn solve_b(&mut self, input: String) -> Result<Answer, String> {
        let bags = input.trim().split('\n');
        let group_priority_sum: Option<u32> = bags
            .collect::<Vec<&str>>()
            .chunks(3)
            .map(|group| {
                group[0].chars().find(|char| {
                    group[1].chars().any(|c2| c2.cmp(char).is_eq())
                        && group[2].chars().any(|c2| c2.cmp(char).is_eq())
                })
            })
            .map(|c| get_item_priority(&c))
            .sum();

        Ok(group_priority_sum.unwrap_or(0).into())
    }
}

static ALPHABET_SIZE: u32 = 26;
fn get_item_priority(c: &Option<char>) -> Option<u32> {
    if c.is_none() {
        return None;
    }
    let char_value: u32 = c.unwrap().to_owned().into();
    let char_lower_a: u32 = 'a'.into();
    let char_upper_a: u32 = 'A'.into();
    if char_value >= char_lower_a {
        return Some(char_value - char_lower_a + 1);
    }
    Some(char_value - char_upper_a + ALPHABET_SIZE + 1)
}

#[cfg(test)]
mod tests {
    use super::Puzzle;
    use common::{Answer, Solution};

    const TEST_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[tokio::test]
    async fn part_a() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_a(String::from(TEST_INPUT)).await,
            Ok(Answer::from(157))
        )
    }

    #[tokio::test]
    async fn part_b() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_b(String::from(TEST_INPUT)).await,
            Ok(Answer::from(70))
        )
    }
}
