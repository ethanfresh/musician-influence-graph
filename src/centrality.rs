use std::collections::{HashMap, VecDeque};
use crate::graph::Graph;

#[derive(Debug, Clone, Copy)]
pub enum CentralityMode {
    Linear,
    Logarithmic,
    InverseSquare,
    WeightedDegree,
    Blended,
}

pub fn compute_centrality(
    graph: &Graph,
    mode: CentralityMode,
    max_depth: Option<usize>,
) -> Vec<(String, f64)> {
    let mut centrality_scores: HashMap<String, f64> = HashMap::new();

    for node in graph.adjacency_list.keys() {
        if graph.adjacency_list.get(node).map_or(0, |n| n.len()) < 2 {
            continue;
        }

        let mut visited = HashMap::new();
        let mut queue = VecDeque::new();

        visited.insert(node.clone(), 0);
        queue.push_back((node.clone(), 0));

        let mut score = 0.0;

        while let Some((current, depth)) = queue.pop_front() {
            if let Some(max) = max_depth {
                if depth > max {
                    continue;
                }
            }

            if let Some(neighbors) = graph.adjacency_list.get(&current) {
                for (neighbor, weight) in neighbors.iter() {
                    if !visited.contains_key(neighbor) {
                        visited.insert(neighbor.clone(), depth + 1);
                        queue.push_back((neighbor.clone(), depth + 1));
                    }

                    let contribution = match mode {
                        CentralityMode::Linear => *weight,
                        CentralityMode::Logarithmic => weight.ln_1p(),
                        CentralityMode::InverseSquare => 1.0 / (depth as f64 + 1.0).powi(2),
                        CentralityMode::WeightedDegree => {
                            let degree = graph.adjacency_list.get(neighbor).map_or(1.0, |n| n.len() as f64);
                            (*weight as f64) * (1.0 / (1.0 + degree)) / (1.0 + depth as f64)
                        },
                        CentralityMode::Blended => {
                            let linear = *weight;
                            let inverse_sq = 1.0 / (depth as f64 + 1.0).powi(2);
                            let log = weight.ln_1p();
                            (linear + inverse_sq + log) / 3.0
                        }
                    };

                    score += contribution;
                }
            }
        }

        centrality_scores.insert(node.clone(), score);
    }

    let mut sorted: Vec<(String, f64)> = centrality_scores.into_iter().collect();
    sorted.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    sorted
}
