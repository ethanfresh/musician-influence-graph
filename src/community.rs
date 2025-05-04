use std::collections::{HashMap, HashSet};
use crate::graph::Graph;

pub fn find_communities(graph: &Graph) -> HashMap<String, usize> {
    let mut community_id = 0;
    let mut visited = HashSet::new();
    let mut communities = HashMap::new();

    for node in graph.edges.keys() {
        if visited.contains(node) || !graph.is_artist(node) {
            continue;
        }

        let mut stack = vec![node.clone()];
        while let Some(current) = stack.pop() {
            if visited.insert(current.clone()) {
                communities.insert(current.clone(), community_id);
                for (neighbor, _) in graph.edges.get(&current).unwrap() {
                    if graph.is_artist(neighbor) && !visited.contains(neighbor) {
                        stack.push(neighbor.clone());
                    }
                }
            }
        }

        community_id += 1;
    }

    communities
}

pub fn top_influential_artists(
    graph: &Graph,
    communities: &HashMap<String, usize>,
    top_n: usize,
) -> Vec<(String, f64, f64)> {
    let mut genre_to_artists: HashMap<String, Vec<&String>> = HashMap::new();

    for (artist, genre) in &graph.genre_by_artist {
        if let Some(_cluster_id) = communities.get(artist) {
            genre_to_artists.entry(genre.clone()).or_default().push(artist);
        }
    }

    let mut all_artists_with_scores: Vec<(String, f64, f64)> = Vec::new();
    for (genre, members) in &genre_to_artists {
        for artist in members {
            if let Some(&length) = graph.lengths_by_artist.get(*artist) {
                let community_size = members.len() as f64;
                let score = (length.ln()) * (community_size.ln());
                all_artists_with_scores.push(((*artist).clone(), length, score));
            }
        }
    }

    all_artists_with_scores.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
    all_artists_with_scores.into_iter().take(top_n).collect()
}

pub fn communities_of_artists<'a>(
    graph: &'a Graph,
    artists: &HashSet<&str>,
) -> HashMap<String, Vec<(&'a str, f64)>> {
    let mut genre_to_artists: HashMap<String, Vec<(&str, f64)>> = HashMap::new();

    for (artist, genre) in &graph.genre_by_artist {
        if artists.contains(artist.as_str()) {
            if let Some(&length) = graph.lengths_by_artist.get(artist) {
                genre_to_artists.entry(genre.clone()).or_default().push((artist, length));
            }
        }
    }

    genre_to_artists
}
