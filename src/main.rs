use std::collections::{HashMap, HashSet};

mod graph;
mod community;

use graph::Graph;
use community::{find_communities, top_influential_artists};

fn main() {
    let graph = Graph::load_from_csv("../wikidata_cleaned.csv");
    let communities = find_communities(&graph);

    // Compute influence scores and genres per community
    let mut community_scores: Vec<(String, f64, f64)> = communities.iter()
        .map(|(genre, members)| {
            let scores: Vec<f64> = members.iter()
                .filter_map(|artist| graph.lengths_by_artist.get(artist).copied())
                .collect();
            let total_score: f64 = scores.iter().sum();
            let average_score = if !scores.is_empty() {
                total_score / scores.len() as f64
            } else {
                0.0
            };
            (genre.clone(), total_score, average_score)
        })
        .collect();

    // Top communities by total influence score
    community_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    println!("Top 10 communities by total influence score:");
    for (genre, total_score, _) in community_scores.iter().take(10) {
        println!("Genre: {} with total influence score: {:.2}", genre, total_score);
    }
    println!("");

    let top_artists = top_influential_artists(&graph, &communities, 10);

    println!("Top 10 artists by influence score:\n-------------------------------");
    for (artist, length, score) in &top_artists {
        println!("{} (length: {:.0}, score: {:.2})", artist, length, score);
    }
    println!("");

    println!("Total number of communities: {}", communities.len());
    println!("Total number of artists: {}", graph.lengths_by_artist.len());
    println!("Total number of edges: {}", graph.edges.len());
    println!("Total number of genres: {}", graph.genre_by_artist.len());
}
