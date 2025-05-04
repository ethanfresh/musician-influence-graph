use std::collections::HashMap;

use crate::graph::Graph;

pub fn find_communities(graph: &Graph) -> HashMap<String, Vec<String>> {
    let mut genre_to_artists: HashMap<String, Vec<String>> = HashMap::new();

    for (artist, genres) in &graph.genre_by_artist {
        for genre in genres {
            genre_to_artists.entry(genre.clone()).or_default().push(artist.clone());
        }
    }

    genre_to_artists
}

pub fn top_influential_artists(
    graph: &Graph,
    _communities: &HashMap<String, Vec<String>>,
    n: usize,
) -> Vec<(String, f64, f64)> {
    let mut scores: Vec<(String, f64, f64)> = graph
        .lengths_by_artist
        .iter()
        .map(|(artist, &length)| {
            let score = (length as f64).log10() * 10.0;
            (artist.clone(), length as f64, score)
        })
        .collect();

    scores.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
    scores.into_iter().take(n).collect()
}
