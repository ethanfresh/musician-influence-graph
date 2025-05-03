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

pub fn print_top_communities(
    graph: &Graph,
    communities: &HashMap<String, usize>,
    top_n: usize,
) {
    let mut genre_to_artists: HashMap<String, Vec<&String>> = HashMap::new();

    for (artist, genre) in &graph.genre_by_artist {
        if let Some(cluster_id) = communities.get(artist) {
            genre_to_artists.entry(genre.clone()).or_default().push(artist);
        }
    }

    let mut sorted_genres: Vec<_> = genre_to_artists.into_iter().collect();
    sorted_genres.sort_by_key(|(_, members)| -(members.len() as isize));

    println!("Top {} artist communities:", top_n);
    for (i, (genre, members)) in sorted_genres.into_iter().take(top_n).enumerate() {
        let mut members_with_lengths: Vec<_> = members
            .iter()
            .filter_map(|artist| graph.lengths_by_artist.get(*artist).map(|&length| (*artist, length)))
            .collect();

        members_with_lengths.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        println!(
            "--- Cluster {} ({}) [Size: {} artists] ---",
            i + 1,
            genre,
            members_with_lengths.len()
        );

        for (artist, length) in members_with_lengths.iter().take(5) {
            println!("{} (length: {:.0})", artist, length);
        }

        println!("...");
    }
}
