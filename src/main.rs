mod graph;
mod dijkstra;
mod centrality;

use graph::Graph;
use centrality::compute_centrality;

fn main() {
    let graph = Graph::from_csv("data/artists.csv");

    println!("Graph loaded with {} nodes.", graph.node_count());

    let centrality_scores = compute_centrality(&graph);

    println!("Top 10 high-priority artists for labeling:");
    for (artist, score) in centrality_scores.iter().take(10) {
        println!("{:<30} | Centrality: {:.4}", artist, score);
    }
}