use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;
use crate::graph::Graph;

#[derive(PartialEq)]
struct State {
    cost: f32,
    node: String,
}

impl Eq for State {}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.partial_cmp(&self.cost).unwrap()
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn shortest_paths(graph: &Graph, start: &str) -> HashMap<String, f32> {
    let mut dist: HashMap<String, f32> = HashMap::new();
    let mut heap = BinaryHeap::new();

    dist.insert(start.to_string(), 0.0);
    heap.push(State { cost: 0.0, node: start.to_string() });

    while let Some(State { cost, node }) = heap.pop() {
        if let Some(neighbors) = graph.neighbors(&node) {
            for (neighbor, weight) in neighbors {
                let next = cost + weight;
                if dist.get(neighbor).map_or(true, |&d| next < d) {
                    dist.insert(neighbor.clone(), next);
                    heap.push(State { cost: next, node: neighbor.clone() });
                }
            }
        }
    }

    dist
}
