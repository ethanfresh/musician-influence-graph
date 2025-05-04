mod graph;
mod community;

use graph::Graph;
use community::{find_communities, top_influential_artists, genre_connectivity_map, prioritized_labeling_targets};

fn main() {
    let graph = Graph::load_from_csv("./wikidata_cleaned.csv");
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

    let top_artists = top_influential_artists(&graph, 10);

    println!("Top 10 artists by influence score:\n-------------------------------");
    for (artist, length, score) in &top_artists {
        println!("{} (length: {:.0}, score: {:.2})", artist, length, score);
    }
    println!("");

    let genre_connections = genre_connectivity_map(&graph);
    let mut connectivity_vec: Vec<(String, usize)> = genre_connections
        .iter()
        .map(|(genre, connected)| (genre.clone(), connected.len()))
        .collect();

    connectivity_vec.sort_by(|a, b| b.1.cmp(&a.1));

    println!("Top 10 most connected genres:\n-----------------------------");
    for (genre, count) in connectivity_vec.iter().take(10) {
        println!("{} connects to {} other genres", genre, count);
    }
    println!("");

    let prioritized_artists = prioritized_labeling_targets(&graph, 10);
    println!("Top 10 artists to prioritize for labeling:\n------------------------------------------");
    for (artist, score) in &prioritized_artists {
        println!("{} (labeling priority score: {:.2})", artist, score);
    }

    println!("");
    println!("Total number of communities: {}", communities.len());
    println!("Total number of artists: {}", graph.lengths_by_artist.len());
    println!("Total number of edges: {}", graph.edges.len());
    println!("Total number of genres: {}", graph.genre_by_artist.len());
} 

#[test]
fn test_genre_connectivity_basic() {
    let mut graph = Graph::new();

    graph.genre_by_artist.insert("Artist1".into(), vec!["Rock".into(), "Pop".into()]);
    graph.genre_by_artist.insert("Artist2".into(), vec!["Rock".into(), "Indie".into()]);
    graph.genre_by_artist.insert("Artist3".into(), vec!["Pop".into(), "Jazz".into()]);

    let connectivity = genre_connectivity_map(&graph);

    // Rock connects to Pop and Indie
    assert!(connectivity.get("Rock").unwrap().contains("Pop"));
    assert!(connectivity.get("Rock").unwrap().contains("Indie"));

    // Pop connects to Rock and Jazz
    assert!(connectivity.get("Pop").unwrap().contains("Rock"));
    assert!(connectivity.get("Pop").unwrap().contains("Jazz"));
}

#[test]
fn test_prioritized_labeling_scoring() {
    let mut graph = Graph::new();

    graph.lengths_by_artist.insert("A1".into(), 1000.0);
    graph.lengths_by_artist.insert("A2".into(), 500.0);

    graph.genre_by_artist.insert("A1".into(), vec!["Rock".into(), "Pop".into()]);
    graph.genre_by_artist.insert("A2".into(), vec!["Rock".into()]);

    let targets = prioritized_labeling_targets(&graph, 2);

    assert_eq!(targets.len(), 2);
    assert!(targets[0].0 == "A1" || targets[1].0 == "A1");
} 