use std::collections::HashSet;

mod graph;
mod community;

use graph::Graph;
use community::{find_communities, top_influential_artists, communities_of_artists};

fn main() {
    let graph = Graph::from_csv("../wikidata_with_lengths_cleaned.csv");
    let communities = find_communities(&graph);
    let top_artists = top_influential_artists(&graph, &communities, 10);

    println!("Top 10 artists by influence score:\n-------------------------------");
    for (artist, length, score) in &top_artists {
        println!("{} (length: {:.0}, score: {:.2})", artist, length, score);
    }
    println!("");

    let artist_set: HashSet<&str> = top_artists.iter().map(|(a, _, _)| a.as_str()).collect();
    let genre_map = communities_of_artists(&graph, &artist_set);

    println!("Communities of top 10 artists:");
    for (genre, members) in genre_map {
        println!("--- Genre: {} ---", genre);
        for (artist, length) in members {
            println!("{} (length: {:.0})", artist, length);
        }
        println!("");
    }
}
