// This module defines the Graph structure used to represent artists,
// their genres, and relationships based on shared genres. It includes
// functionality to build the graph from a CSV file.

use std::collections::HashMap;
use csv::ReaderBuilder;

/// Graph represents a network of artists and genres with associated data
///
/// - edges: Connections between artists and genres or between artists with shared genres
/// - lengths_by_artist: Stores the length data for each artist
/// - genre_by_artist: Maps each artist to a list of their genres
pub struct Graph {
    pub edges: HashMap<String, Vec<(String, f64)>>,
    pub lengths_by_artist: HashMap<String, f64>,
    pub genre_by_artist: HashMap<String, Vec<String>>,
}

impl Graph {
    /// Creates an empty Graph
    /// Returns a new Graph instance with empty maps
    pub fn new() -> Self {
        Graph {
            edges: HashMap::new(),
            lengths_by_artist: HashMap::new(),
            genre_by_artist: HashMap::new(),
        }
    }

    /// Constructs the Graph from a CSV file
    /// # Inputs
    /// File path to the CSV input file. Expected format: artist, genres, length
    /// # Outputs
    /// A graph of artists, genres, and their connections
    pub fn load_from_csv(path: &str) -> Self {
        let mut graph = Graph::new();

        // Initialize CSV reader with headers
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

            // Skip invalid or zero-length artists
            if artist.is_empty() || length <= 0.0 {
                continue;
            }

            // Parse genres from CSV
            let genres: Vec<String> = genres_str
                .trim_matches(|c| c == '[' || c == ']')
                .split(',')
                .map(|s| s.trim().trim_matches('\'').to_string())
                .filter(|s| !s.is_empty())
                .collect();

            graph.lengths_by_artist.insert(artist.clone(), length);

            // Create edges between artist and each of their genres
            for genre in &genres {
                graph.edges.entry(artist.clone()).or_default().push((genre.clone(), 1.0));
                graph.edges.entry(genre.clone()).or_default().push((artist.clone(), 1.0));
            }

            graph.genre_by_artist.insert(artist.clone(), genres);
            artists.push(artist);
        }

        let mut link_count = 0;

        // Create edges between artists who share at least one genre
        for i in 0..artists.len() {
            for j in (i + 1)..artists.len() {
                let a1 = &artists[i];
                let a2 = &artists[j];

                // Get genres for each artist
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

                // Count shared genres
                let shared = g1_vec.iter().filter(|g| g2_vec.contains(g)).count();

                if shared >= 1 {
                    // Add weighted edge between artists
                    graph.edges.entry(a1.clone()).or_default().push((a2.clone(), shared as f64));
                    graph.edges.entry(a2.clone()).or_default().push((a1.clone(), shared as f64));
                    link_count += 1;
                }
            }
        }

        println!("Total artist-artist links created: {}", link_count);

        graph
    }
}
