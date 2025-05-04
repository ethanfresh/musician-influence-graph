use std::collections::HashMap;
use csv::ReaderBuilder;

pub struct Graph {
    pub edges: HashMap<String, Vec<(String, f64)>>,
    pub lengths_by_artist: HashMap<String, f64>,
    pub genre_by_artist: HashMap<String, Vec<String>>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            edges: HashMap::new(),
            lengths_by_artist: HashMap::new(),
            genre_by_artist: HashMap::new(),
        }
    }

    pub fn load_from_csv(path: &str) -> Self {
        let mut graph = Graph::new();
        let mut rdr = ReaderBuilder::new()
            .has_headers(true)
            .from_path(path)
            .expect("Cannot read CSV file");

        let mut artists = Vec::new();

        for result in rdr.records() {
            let record = result.expect("Error reading record");
            let artist = record.get(0).unwrap_or("").trim().to_string();
            let genres_str = record.get(1).unwrap_or("").trim();
            let length: f64 = record.get(2).unwrap_or("0").parse().unwrap_or(0.0);

            if artist.is_empty() || length <= 0.0 {
                continue;
            }

            let genres: Vec<String> = genres_str
                .trim_matches(|c| c == '[' || c == ']')
                .split(',')
                .map(|s| s.trim().trim_matches('\'').to_string())
                .filter(|s| !s.is_empty())
                .collect();

            graph.lengths_by_artist.insert(artist.clone(), length);

            for genre in &genres {
                graph.edges.entry(artist.clone()).or_default().push((genre.clone(), 1.0));
                graph.edges.entry(genre.clone()).or_default().push((artist.clone(), 1.0));
            }

            graph.genre_by_artist.insert(artist.clone(), genres);
            artists.push(artist);
        }

        // Connect artists who share at least 1 genre
        let mut link_count = 0;
        for i in 0..artists.len() {
            for j in (i + 1)..artists.len() {
                let a1 = &artists[i];
                let a2 = &artists[j];

                let g1_vec_owned;
                let g2_vec_owned;
                let g1_vec = match graph.genre_by_artist.get(a1) {
                    Some(v) => v,
                    None => {
                        g1_vec_owned = Vec::new();
                        &g1_vec_owned
                    }
                };
                let g2_vec = match graph.genre_by_artist.get(a2) {
                    Some(v) => v,
                    None => {
                        g2_vec_owned = Vec::new();
                        &g2_vec_owned
                    }
                };

                let shared = g1_vec.iter().filter(|g| g2_vec.contains(g)).count();
                if shared >= 1 {
                    graph.edges.entry(a1.clone()).or_default().push((a2.clone(), shared as f64));
                    graph.edges.entry(a2.clone()).or_default().push((a1.clone(), shared as f64));
                    link_count += 1;
                }
            }
        }

        println!("Total artist-artist links created: {}", link_count);

        graph
    }

    pub fn is_artist(&self, node: &str) -> bool {
        self.lengths_by_artist.contains_key(node)
    }

    pub fn get_genres(&self, artist: &str) -> Vec<String> {
        self.genre_by_artist.get(artist).cloned().unwrap_or_default()
    }
}
