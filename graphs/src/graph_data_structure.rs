use std::collections::HashMap;

struct Graph {
    adjacency_list: HashMap<String, Vec<String>>
}

impl Graph {
    fn new() -> Self{
        Graph{
            adjacency_list: HashMap::new(),
        }
    }
    fn add_edge(&mut self, u: &str, v: &str) {
        self.adjacency_list
        .entry(u.to_string())
        .or_default()
        .push(v.to_string());
        self.adjacency_list
        .entry(v.to_string())
        .or_default()
        .push(u.to_string());
    }
}
// A --> B
// A --> C 
/*

            A
           / \
           B C
*/
pub fn exec() {
    let mut graph = Graph::new();
    graph.add_edge("A", "B");
    graph.add_edge("A", "C");
    println!("{:?}", graph.adjacency_list);
}
