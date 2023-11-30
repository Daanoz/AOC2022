use std::{str::FromStr, collections::{HashMap, HashSet, VecDeque}};

use async_trait::async_trait;
use common::{Answer, Solution};
use regex::Regex;

pub struct Puzzle {
    max_geode_increment: Vec<i32>,
}

type Costs = [i32; 4];
type OreStash = [i32; 4];
type BotCount = [i32; 4];
type QueueItem = (OreStash, BotCount, i32);

struct Blueprint {
    id: i32,
    bot_costs: [Costs; 4],
    max_cost: Costs
}
impl FromStr for Blueprint {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = Regex::new(
            r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").expect("Valid regex").captures(s)
        .expect("At least one match");
        fn get_capture_as_i32(captures: &regex::Captures, index: usize) -> i32 {
            captures
                .get(index)
                .expect(&format!("Should have an match at index: {}", index))
                .as_str()
                .parse()
                .expect(&format!("Parsable as i32 at index: {}", index))
        }
        Ok(Self {
            id: get_capture_as_i32(&captures, 1),
            bot_costs: [
            [get_capture_as_i32(&captures, 2), 0, 0, 0],
            [get_capture_as_i32(&captures, 3), 0, 0, 0],
            [
                get_capture_as_i32(&captures, 4),
                get_capture_as_i32(&captures, 5),
                0,
                0,
            ],
            [
                get_capture_as_i32(&captures, 6),
                0,
                get_capture_as_i32(&captures, 7),
                0,
            ]],
            max_cost: [
                get_capture_as_i32(&captures, 2).max(get_capture_as_i32(&captures, 3).max(get_capture_as_i32(&captures, 4).max(get_capture_as_i32(&captures, 6)))),
                get_capture_as_i32(&captures, 5),
                get_capture_as_i32(&captures, 7),
                0,
            ],
        })
    }
}

impl Default for Puzzle {
    fn default() -> Self {
        let mut max_geode_increment = vec![0 as i32; 33];
        for m in 1..=32 {
            max_geode_increment[m] = max_geode_increment[m - 1] + (m as i32);
        }
        Self {
            max_geode_increment
        }
    }
}

impl Puzzle {
    fn find_best_outcome_for_blueprint(&self, bp: &Blueprint, max_time: i32) -> i32 {
        let mut queue: VecDeque<QueueItem> = vec![(
            [0, 0, 0, 0] as OreStash,
            [1, 0, 0, 0] as BotCount,
            0
        )].into();
        let mut visits: Vec<HashMap<BotCount, HashSet<OreStash>>> = vec![HashMap::new(); (max_time + 1) as usize];
        let mut max_geode_count = 0;
        while let Some(current) = queue.pop_back() {
            if current.2 >= max_time {
                max_geode_count = max_geode_count.max(current.0[3]);
                continue
            }
            let time_left = max_time - current.2;

            let best_case_max = self.max_geode_increment[time_left as usize] + current.0[3] + (current.1[3] * time_left);
            if max_geode_count > best_case_max {
                continue
            }

            let time_cache = visits.get_mut(current.2 as usize).expect("Time cache exists");
            if let Some(bot_cache) = time_cache.get_mut(&current.1) {
                if bot_cache.contains(&current.0) {
                    continue
                }
                bot_cache.insert(current.0);
            } else {
                time_cache.insert(current.1, HashSet::from([current.0]));
            }
            let mut options = find_bot_purchase_options(&current, bp);
            queue.append(&mut options);
        }
        max_geode_count
    }
}

fn find_bot_purchase_options((ores, bots, time): &QueueItem, bp: &Blueprint) -> VecDeque<QueueItem> {
    let mut options: VecDeque<QueueItem> = vec![].into();
    for bot in (0..=3).rev() {
        if bot != 3 && (bp.max_cost[bot] <= bots[bot]) { // we don't need more of these!
            continue
        }
        let cost = bp.bot_costs[bot];
        if ores.iter().enumerate().all(|(i, o)| o >= &cost[i]) {
            let mut new_bots = bots.clone();
            new_bots[bot] = new_bots[bot] + 1;
            options.push_back((
                ores.iter().enumerate().map(|(i, o)| (o + bots[i]) - cost[i]).collect::<Vec<_>>().try_into().unwrap(),
                new_bots,
                time + 1
            ));
            if bot == 3 { // we can buy a geode bot, ignore all other options.
                return options
            }
        }
    }
    if ores[0] <= bp.max_cost[0] {
        options.push_front((
            ores.iter().enumerate().map(|(i, o)| o + bots[i]).collect::<Vec<_>>().try_into().unwrap(),
            bots.clone(),
            time + 1
        ))
    }
    options
}

#[async_trait]
impl Solution for Puzzle {
    async fn solve_a(&mut self, input: String) -> Result<Answer, String> {
        let blueprints = input
            .lines()
            .map(|line| line.parse::<Blueprint>().expect("Parsable as Blueprint"))
            .collect::<Vec<_>>();

        Answer::from(blueprints.into_iter().fold(0, |sum, bp| {
            let output = self.find_best_outcome_for_blueprint(&bp, 24);
            sum + (output * bp.id)
        })).into()
    }

    async fn solve_b(&mut self, input: String) -> Result<Answer, String> {
        let blueprints = input
            .lines()
            .map(|line| line.parse::<Blueprint>().expect("Parsable as Blueprint"))
            .take(3)
            .collect::<Vec<_>>();

        Answer::from(blueprints.into_iter().fold(1, |sum, bp| {
            let output = self.find_best_outcome_for_blueprint(&bp, 32);
            sum * output
        })).into()
    }
}

#[cfg(test)]
mod tests {
    use super::Puzzle;
    use common::{Answer, Solution};

    const TEST_INPUT: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

    #[tokio::test]
    async fn part_a() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_a(String::from(TEST_INPUT)).await,
            Ok(Answer::from(33))
        )
    }

    #[tokio::test]
    async fn part_b() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_b(String::from(TEST_INPUT)).await,
            Ok(Answer::from(56 * 62))
        )
    }
}
