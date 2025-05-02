mod graph;
mod dijkstra;
mod prioritizer;

use graph::Graph;
use prioritizer::select_influential_nodes;

fn main() {
    let graph = Graph::from_csv("data/artists.csv");

    println!("Graph loaded with {} nodes.", graph.node_count());

    let prioritized = select_influential_nodes(&graph, 100);

    println!("Top 100 high-priority artists for labeling:");
    for artist in prioritized {
        println!("{}", artist);
    }
}