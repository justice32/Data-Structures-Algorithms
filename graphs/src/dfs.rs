use std::{collections::HashMap, vec};

fn dfs<'a>(graph: &HashMap<&'a str, Vec<&'a str>>, start: &'a str, visited: &mut Vec<&'a str>) {
    if visited.contains(&start) {
        return;
    }
    visited.push(start);
    println!("Visited: {}", start);

    if let Some(neighbors) = graph.get(start) {
        for neigbor in neighbors {
            dfs(graph, neigbor, visited);
        }
    }
}

pub fn exec() {
    let graph = HashMap::from([
        ("A", vec!["B", "C"]),
        ("B", vec!["D", "E"]),
        ("C", vec!["F"]),
        ("D", vec![]),
        ("E", vec!["F"]),
        ("F", vec![]),
    ]);

    let mut visited = vec![];
    dfs(&graph, "A", &mut visited);
}
