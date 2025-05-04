// This module provides functions to analyze community structure, influence,
// and connectivity among artists and genres in a music graph.

use std::collections::{HashMap, HashSet};
use crate::graph::Graph;

/// Groups artists into communities based on shared genres
/// # Inputs
/// - graph: Reference to a Graph that contains artist-genre mappings
/// # Outputs
/// - HashMap<String, Vec<String>>: A map from each genre to a list of artists in that genre
pub fn find_communities(graph: &Graph) -> HashMap<String, Vec<String>> {
    let mut genre_to_artists: HashMap<String, Vec<String>> = HashMap::new();

    for (artist, genres) in &graph.genre_by_artist {
        for genre in genres {
            // Add artist to the list of artists for each of their genres
            genre_to_artists.entry(genre.clone()).or_default().push(artist.clone());
        }
    }

    genre_to_artists
}

/// Identifies the top n influential artists based on their song lengths
/// # Inputs
/// - graph: Reference to the Graph struct
/// - n: Number of top artists to return
/// # Outputs
/// - Vec<(String, f64, f64)>: List of tuples (artist name, length, influence score)
pub fn top_influential_artists(
    graph: &Graph,
    n: usize,
) -> Vec<(String, f64, f64)> {
    let mut scores: Vec<(String, f64, f64)> = graph
        .lengths_by_artist
        .iter()
        .map(|(artist, &length)| {
            // Calculate influence score based on log-scaled length
            let score = (length as f64).log10() * 10.0;
            (artist.clone(), length as f64, score)
        })
        .collect();

    // Sort by influence score descending 
    scores.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());

    // Return the top n artists
    scores.into_iter().take(n).collect()
}

/// Builds a map of genre connectivity based on shared artists between genres
/// # Inputs
/// - graph: Reference to the Graph struct
/// # Outputs
/// - HashMap<String, HashSet<String>>: For each genre, lists genres it connects to via shared artists
pub fn genre_connectivity_map(graph: &Graph) -> HashMap<String, HashSet<String>> {
    let mut genre_to_artists: HashMap<String, HashSet<String>> = HashMap::new();

    // Build genre -> artists map
    for (artist, genres) in &graph.genre_by_artist {
        for genre in genres {
            genre_to_artists.entry(genre.clone()).or_default().insert(artist.clone());
        }
    }

    let mut genre_connections: HashMap<String, HashSet<String>> = HashMap::new();

    let genres: Vec<String> = genre_to_artists.keys().cloned().collect();

    // Compare each genre pair to check if they share any artists
    for i in 0..genres.len() {
        for j in (i + 1)..genres.len() {
            let g1 = &genres[i];
            let g2 = &genres[j];

            let artists1 = &genre_to_artists[g1];
            let artists2 = &genre_to_artists[g2];

            // If they share at least 1 artist, mark as connected
            if !artists1.is_disjoint(artists2) {
                genre_connections.entry(g1.clone()).or_default().insert(g2.clone());
                genre_connections.entry(g2.clone()).or_default().insert(g1.clone());
            }
        }
    }

    genre_connections
}

/// Identifies high-priority artists for labeling based on influence and genre connectivity
/// # Inputs
/// - graph: Reference to the Graph struct
/// - n: Number of artists to return
/// # Outputs
/// - Vec<(String, f64)>: List of (artist name, priority score)
pub fn prioritized_labeling_targets(
    graph: &Graph,
    n: usize,
) -> Vec<(String, f64)> {
    let genre_connections = genre_connectivity_map(graph);
    let empty_vec = Vec::new();

    let mut artist_priority: Vec<(String, f64)> = graph.lengths_by_artist.iter().map(|(artist, &length)| {
        let genres = graph.genre_by_artist.get(artist).unwrap_or(&empty_vec);
        
        // Sum of connected genres for each genre this artist belongs to
        let connectivity: usize = genres.iter()
            .map(|g| genre_connections.get(g).map_or(0, |set| set.len()))
            .sum();

        // Prioritize based on both connectivity and influence
        let score = (length.log10() * 10.0) * connectivity as f64;
        (artist.clone(), score)
    }).collect();

    // Sort artists by descending score
    artist_priority.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    artist_priority.into_iter().take(n).collect()
}
