use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

use async_trait::async_trait;
use common::{Answer, Solution};
use regex::Regex;

pub struct Puzzle {}

impl Default for Puzzle {
    fn default() -> Self {
        Self {}
    }
}

#[derive(Debug)]
struct Node {
    name: String,
    flow_rate: u32,
    neighbors: Vec<NeighborRef>,
}
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
type NodeRef = Rc<RefCell<Node>>;
type NeighborRef = Weak<RefCell<Node>>;
type NodeMap = HashMap<String, NodeRef>;

fn parse_input(data: String) -> NodeMap {
    let mut nodes: NodeMap = NodeMap::new();
    let regex =
        Regex::new(r"Valve ([A-Z]{2}) has flow rate=(\d+); tunnels? leads? to valves? ([A-Z, ]+)")
            .expect("Valid regex");
    let rows: Vec<(&str, &str, &str)> = data
        .split("\n")
        .map(|row| {
            let captures = regex.captures(row).expect(&format!("At least one match: {}", row));
            (
                captures.get(1).expect("Valve name").as_str(),
                captures.get(2).expect("Flow rate").as_str(),
                captures.get(3).expect("Neighbors").as_str(),
            )
        })
        .collect();
    for (name, flow_rate, _) in rows.clone() {
        nodes.insert(
            name.into(),
            Rc::new(RefCell::new(Node {
                name: name.into(),
                flow_rate: flow_rate.parse().expect("Flow rate as number"),
                neighbors: vec![],
            })),
        );
    }
    for (name, _, neighbors) in rows {
        let neighbors = neighbors
            .split(", ")
            .map(|n| Rc::downgrade(nodes.get(n).expect("Neighbor should be in map")))
            .collect::<Vec<NeighborRef>>();
        nodes
            .get(name)
            .expect("Node should be in map")
            .borrow_mut()
            .neighbors = neighbors;
    }
    nodes
}

fn calculate_travel_distances(nodes: &NodeMap) -> HashMap<String, HashMap<String, u32>> {
    let starting_nodes = nodes
        .values()
        .filter(|n| {
            let node = n.borrow();
            node.flow_rate > 0 || node.name == "AA"
        })
        .map(|n| n.clone())
        .collect::<Vec<NodeRef>>();
    let mut travel_distances = HashMap::<String, HashMap<String, u32>>::new();
    starting_nodes.iter().for_each(|node| {
        let node_name = node.borrow().name.clone();
        let mut distance_map = HashMap::<String, u32>::new();
        distance_map.insert(node_name.clone(), 0);
        let mut visited: Vec<NodeRef> = vec![];
        let mut queue: Vec<NodeRef> = vec![node.clone()];
        while queue.len() > 0 {
            let current = queue.remove(0);
            if visited.contains(&&current) {
                continue;
            }
            visited.push(current.clone());
            let current = current.borrow();
            let current_distance = distance_map.get(&current.name).expect("Node should be in distance_map").to_owned();

            current
                .neighbors
                .iter()
                .map(|n| n.upgrade().expect("Upgrade to RC"))
                .filter(|n| !visited.contains(&n))
                .for_each(|n| {
                    let target_name = n.borrow().name.clone();
                    let distance = (current_distance + 1)
                        .min(*distance_map.get(&target_name).unwrap_or(&u32::MAX));
                    distance_map.insert(target_name, distance);
                    queue.push(n);
                });
        }
        distance_map.remove(&node.borrow().name);
        travel_distances.insert(node_name, distance_map);
    });
    travel_distances
}

struct DFS {
    memo: HashMap<u32, HashMap<String, HashMap<String, u32>>>,
    node_map: NodeMap,
    time_limit: u32,
}

impl DFS {
    fn new(node_map: NodeMap) -> Self {
        Self {
            memo: HashMap::new(),
            node_map,
            time_limit: 30,
        }
    }

    fn get_memo_key(
        &self,
        open_nodes: &[&NodeRef],
        current_node: &NodeRef,
        minutes: u32,
    ) -> (u32, String, String) {
        let mut open_nodes_ids = open_nodes
            .iter()
            .map(|on| on.borrow().name.clone())
            .collect::<Vec<String>>();
        open_nodes_ids.sort();
        let open_nodes_ids = open_nodes_ids.join(",");
        let current_node_id = current_node.borrow().name.clone();
        let cached_mins = minutes;
        (cached_mins, current_node_id, open_nodes_ids)
    }

    fn read_memo(&self, memo_key: &(u32, String, String)) -> Option<u32> {
        self.memo
            .get(&memo_key.0)
            .and_then(|m| m.get(&memo_key.1))
            .and_then(|m| m.get(&memo_key.2))
            .copied()
    }

    fn write_memo(&mut self, memo_key: (u32, String, String), value: u32) -> () {
        self.memo
            .entry(memo_key.0)
            .or_insert(HashMap::new())
            .entry(memo_key.1)
            .or_insert(HashMap::new())
            .insert(memo_key.2, value);
    }

    pub fn run(&mut self, current: &NodeRef, open_nodes: &[&NodeRef], minutes: u32) -> u32 {
        let minutes = minutes + 1;
        if minutes >= self.time_limit || (open_nodes.len() == self.node_map.len()) {
            return 0;
        }
        let memo_key = self.get_memo_key(open_nodes, &current, minutes);
        let cached = self.read_memo(&memo_key);
        if let Some(cached) = cached {
            return cached;
        }

        let mut max = 0;
        max = max.max(
            current
                .borrow()
                .neighbors
                .iter()
                .map(|n| self.run(&n.upgrade().expect("Upgrade to RC"), open_nodes, minutes))
                .max()
                .expect("At least one neighbor"),
        );
        let current_flow_rate = current.borrow().flow_rate;
        if current_flow_rate > 0 && !open_nodes.contains(&current) {
            let extra_pressure = current_flow_rate * (self.time_limit - minutes);
            let mut open_nodes_with_current = open_nodes.clone().to_vec();
            open_nodes_with_current.push(current);
            max = max.max(extra_pressure + self.run(current, &open_nodes_with_current, minutes));
        }

        self.write_memo(memo_key, max);
        return max;
    }
}

fn find_all_possible_paths(node_map: NodeMap) -> Vec<(u32, Vec<NodeRef>)> {
    let travel_distances = calculate_travel_distances(&node_map);
    let time_limit: u32 = 26;
    let mut queue = vec![(
        node_map
            .get("AA")
            .expect("Starting node should be present")
            .clone(),
        node_map
            .values()
            .filter(|n| n.borrow().flow_rate > 0)
            .map(|n| n.clone())
            .collect::<Vec<NodeRef>>(),
        time_limit,
        vec![],
        0 as u32,
    )];
    let mut paths: Vec<(u32, Vec<NodeRef>)> = vec![];
    while !queue.is_empty() {
        let (node, next, time, road, pressure) = queue.remove(0);
        let distances = travel_distances
            .get(&node.borrow().name)
            .expect("Node should be in travel_distances");
        let next_visits = next
            .iter()
            .filter(|n| {
                time as i64 - *distances
                    .get(&n.borrow().name)
                    .expect("Node should be in distances") as i64
                    > 1
            })
            .map(|n| n.clone())
            .collect::<Vec<NodeRef>>();
        if next_visits.is_empty() {
            paths.push((pressure, road));
            continue;
        }
        for next_visit in next_visits {
            let time_left = time
                - distances
                    .get(&next_visit.borrow().name)
                    .expect("Node should be in distances")
                - 1;
            let next_pressure = pressure + time_left * next_visit.borrow().flow_rate;
            let mut next_road = road.clone();
            next_road.push(next_visit.clone());
            queue.push((
                next_visit.clone(),
                next.iter()
                    .filter(|n| n.borrow().name != next_visit.borrow().name)
                    .map(|n| n.clone())
                    .collect::<Vec<NodeRef>>(),
                time_left,
                next_road.clone(),
                next_pressure,
            ));
            paths.push((next_pressure, next_road));
        }
    }
    paths
}

fn find_best_result_with_2(mut paths: Vec<(u32, Vec<NodeRef>)>) -> u32 {
    paths.sort_by(|a, b| b.0.cmp(&a.0));

    let single_max = paths[0].0;
    let mut max = u32::MIN;
    for (index, path) in paths.iter().enumerate() {
        if path.0 + single_max < max {
            continue; // this trick safes more than 9 seconds, or 90% of this function
        } 
        let first_path_not_reaching_max = paths.iter().position(|p| (path.0 + p.0) < max).unwrap_or(paths.len());
        if first_path_not_reaching_max < index + 1 {
            continue;
        }
        paths[index + 1..first_path_not_reaching_max].iter().for_each(|path_b| {
            let combined_pressure = path.0 + path_b.0;
            if combined_pressure > max {
                if path.1.iter().all(|s| !path_b.1.contains(s)) {
                    max = combined_pressure;
                }
            }
        });
    }
    max
}

#[async_trait]
impl Solution for Puzzle {
    async fn solve_a(&mut self, input: String) -> Result<Answer, String> {
        let nodes: NodeMap = parse_input(input);
        let mut dfs = DFS::new(nodes.clone());

        let start_node = nodes.get("AA").expect("Starting node should be present");
        let open_nodes = nodes
            .values()
            .filter(|n| n.borrow().flow_rate == 0)
            .collect::<Vec<&NodeRef>>();
        let result = dfs.run(start_node, &open_nodes, 0);
        Answer::from(result).into()
    }

    async fn solve_b(&mut self, input: String) -> Result<Answer, String> {
        let nodes: NodeMap = parse_input(input);
        let paths = find_all_possible_paths(nodes);

        Answer::from(find_best_result_with_2(paths)).into()
    }
}

#[cfg(test)]
mod tests {
    use super::Puzzle;
    use common::{Answer, Solution};

    const TEST_INPUT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    #[tokio::test]
    async fn part_a() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_a(String::from(TEST_INPUT)).await,
            Ok(Answer::from(1651))
        )
    }

    #[tokio::test]
    async fn part_b() {
        let mut puzzle = Puzzle::default();
        assert_eq!(
            puzzle.solve_b(String::from(TEST_INPUT)).await,
            Ok(Answer::from(1707))
        )
    }
}
