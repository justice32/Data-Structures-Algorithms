use std::collections::HashMap;

/// Bellman-Ford Algorithm Implementation
/// 
/// Finds the shortest distance from a starting vertex to all other vertices in a graph,
/// even if the graph has negative edge weights.
///
/// Parameters:
/// - `edges`: A vector of tuples representing the graph edges in the form (u, v, weight),
///    where `u` is the source vertex, `v` is the destination vertex, and `weight` is the cost of the edge.
/// - `vertices`: A list of all vertices in the graph.
/// - `start`: The starting vertex for the shortest path calculation.
///
/// Returns:
/// A HashMap where the keys are vertex names and the values are the shortest distances
/// from the starting vertex to that vertex.
fn bellman_ford(edges: &Vec<(&str, &str, isize)>, vertices: Vec<&str>, start: &str) -> HashMap<String, isize> {
    // Initialize a HashMap to store the shortest distances to each vertex.
    // Initially, set the distance to all vertices as "infinity" (isize::MAX).
    let mut distances = HashMap::new();

    // Populate the distances map with each vertex, setting the distance to infinity.
    for vertex in vertices.iter() {
        distances.insert(vertex.to_string(), isize::MAX);
    }
    // Set the distance to the starting vertex to 0 (distance to itself is 0).
    distances.insert(start.to_string(), 0);

    // Relax all edges (vertices.len() - 1) times.
    // This is because the shortest path in a graph with `n` vertices can have at most `n-1` edges.
    for _ in 0..vertices.len() - 1 {
        // Iterate through each edge in the graph.
        for &(u, v, weight) in edges {
            // Get the current distance to the source vertex (`u`).
            let distance_u = *distances.get(u).unwrap_or(&isize::MAX);
            // Get the current distance to the destination vertex (`v`).
            let distance_v = *distances.get(v).unwrap_or(&isize::MAX);

            // Check if a shorter path can be found via edge (u -> v).
            if distance_u != isize::MAX && distance_u + weight < distance_v {
                // Update the shortest distance to vertex `v`.
                distances.insert(v.to_string(), distance_u + weight);
            }
        }
    }

    // Return the final distances map.
    distances
}

pub fn exec() {
    // Define the edges of the graph as a vector of tuples.
    // Each tuple represents (source vertex, destination vertex, weight of the edge).
    let edges = vec![
        ("A", "B", 1),    // Edge from A to B with weight 1
        ("B", "C", 2),    // Edge from B to C with weight 2
        ("A", "C", 4),    // Edge from A to C with weight 4
        ("C", "D", 1),    // Edge from C to D with weight 1
        ("D", "B", -6),   // Edge from D to B with weight -6 (negative weight)
    ];

    // Define the vertices in the graph.
    let vertices = vec!["A", "B", "C", "D"];

    // Run the Bellman-Ford algorithm starting from vertex "A".
    let distances = bellman_ford(&edges, vertices, "A");

    // Print the resulting shortest distances from "A" to all other vertices.
    println!("{:?}", distances);
}
