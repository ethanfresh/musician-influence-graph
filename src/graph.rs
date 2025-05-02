use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use csv::Reader;

pub struct Graph {
    pub adjacency_list: HashMap<String, Vec<(String, f32)>>,
}

impl Graph {
    pub fn new() -> Self {
        Graph { adjacency_list: HashMap::new() }
    }

    pub fn from_csv(path: &str) -> Self {
        let mut graph = Graph::new();
        let file = File::open(path).expect("Could not open file");
        let mut rdr = Reader::from_reader(BufReader::new(file));

        for result in rdr.records() {
            let record = result.expect("Failed to read record");
            let from = record[0].to_string();
            let to = record[1].to_string();
            let weight: f32 = record[2].parse().unwrap_or(1.0);

            graph.adjacency_list.entry(from.clone()).or_default().push((to.clone(), weight));
            graph.adjacency_list.entry(to).or_default().push((from, weight)); // undirected
        }

        graph
    }

    pub fn node_count(&self) -> usize {
        self.adjacency_list.len()
    }

    pub fn neighbors(&self, node: &str) -> Option<&Vec<(String, f32)>> {
        self.adjacency_list.get(node)
    }
}