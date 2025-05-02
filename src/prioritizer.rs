use std::collections::{HashMap, HashSet};
use crate::graph::Graph;

pub fn select_influential_nodes(graph: &Graph, k: usize) -> Vec<String> {
    let mut selected = Vec::new();
    let mut covered = HashSet::new();

    for _ in 0..k {
        let mut best_node = None;
        let mut best_gain = 0;

        for node in graph.adjacency_list.keys() {
            if selected.contains(node) {
                continue;
            }

            let new_coverage = graph.neighbors(node)
                .unwrap_or(&vec![])
                .iter()
                .filter(|(n, _)| !covered.contains(n))
                .count();

            if new_coverage > best_gain {
                best_gain = new_coverage;
                best_node = Some(node);
            }
        }

        if let Some(node) = best_node {
            selected.push(node.clone());
            if let Some(neighbors) = graph.neighbors(node) {
                for (neighbor, _) in neighbors {
                    covered.insert(neighbor.clone());
                }
            }
        } else {
            break;
        }
    }

    selected
}
