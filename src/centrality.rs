use std::collections::HashMap;
use crate::graph::Graph;
use crate::dijkstra::shortest_paths;

pub fn compute_centrality(graph: &Graph) -> Vec<(String, f32)> {
    let mut scores = Vec::new();

    for node in graph.adjacency_list.keys() {
        let distances = shortest_paths(graph, node);
        let total_distance: f32 = distances.values().sum();
        let avg_distance = total_distance / distances.len() as f32;
        let centrality_score = 1.0 / avg_distance;
        scores.push((node.clone(), centrality_score));
    }

    scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    scores
}
