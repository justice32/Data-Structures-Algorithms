use std::{collections::{HashMap, VecDeque}, vec};
fn bfs(graph: &HashMap<&str, Vec<&str>>, start: &str) {
    let mut visited = vec![]; // 
    let mut queue = VecDeque::new(); // new queue 

    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        if !visited.contains(&node) {
            visited.push(node);
            println!("Visited: {}", node);

            if let Some(neighbors) = graph.get(node) {
                for neighbor in neighbors {
                    queue.push_back(neighbor);
                }
            }
        }
    }
}

        /*

            A
          /  \
          B   C
        /  \   \
       D    E<->F
      / \    
         */

pub fn exec() {
    let graph = HashMap::from([
        ("A", vec!["B", "C"]),
        ("B", vec!["D", "E"]),
        ("C", vec!["F"]),
        ("D", vec![]),
        ("E", vec!["F"]),
        ("F", vec![]),
    ]);
    bfs(&graph, "A");
}
