mod graph;
mod community;

use graph::Graph;
use community::{find_communities, print_top_communities};

fn main() {
    let graph = Graph::from_csv("../wikidata_with_lengths_cleaned.csv");
    let communities = find_communities(&graph);
    print_top_communities(&graph, &communities, 5);
}
