use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Graph {
    pub edges: HashMap<String, Vec<(String, f64)>>,
    pub genre_by_artist: HashMap<String, String>,
    pub lengths_by_artist: HashMap<String, f64>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            edges: HashMap::new(),
            genre_by_artist: HashMap::new(),
            lengths_by_artist: HashMap::new(),
        }
    }

    pub fn from_csv(filename: &str) -> Self {
        let file = File::open(filename).expect("Cannot open file");
        let reader = BufReader::new(file);

        let mut artist_to_genres: HashMap<String, HashSet<String>> = HashMap::new();
        let mut genre_lengths: HashMap<String, f64> = HashMap::new();
        let mut lengths_by_artist: HashMap<String, f64> = HashMap::new();
        let mut genre_by_artist: HashMap<String, String> = HashMap::new();
        let mut lines = reader.lines();
        lines.next();

        for line in lines.flatten() {
            let parts: Vec<&str> = line.split(',').map(|s| s.trim()).collect();
            if parts.len() < 4 {
                continue;
            }

            let artist = parts[0].to_string(); // artistLabel
            let genre = parts[1].to_string();  // genreLabel
            let length: f64 = parts[3].parse().unwrap_or(0.0); // length

            if artist.is_empty() || genre.is_empty() || length == 0.0 {
                continue;
            }

            artist_to_genres.entry(artist.clone()).or_default().insert(genre.clone());
            genre_lengths.entry(genre.clone()).or_insert(length);
            genre_by_artist.insert(artist.clone(), genre);
            lengths_by_artist.insert(artist.clone(), length);
        }

        let mut graph = Graph {
            edges: HashMap::new(),
            genre_by_artist,
            lengths_by_artist,
        };

        let mut genre_to_artists: HashMap<String, Vec<String>> = HashMap::new();

        for (artist, genres) in &artist_to_genres {
            for genre in genres {
                genre_to_artists.entry(genre.clone()).or_default().push(artist.clone());
            }
        }

        for (genre, artists) in genre_to_artists {
            if let Some(&length) = genre_lengths.get(&genre) {
                for i in 0..artists.len() {
                    for j in (i + 1)..artists.len() {
                        let a1 = &artists[i];
                        let a2 = &artists[j];
                        graph.edges.entry(a1.clone()).or_default().push((a2.clone(), length));
                        graph.edges.entry(a2.clone()).or_default().push((a1.clone(), length));
                    }
                }
            }
        }

        graph
    }

    pub fn is_artist(&self, node: &str) -> bool {
        self.edges.contains_key(node)
    }
}
