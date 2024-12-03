use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::cmp::Ordering;
use std::hash::Hash;


#[derive(Eq, PartialEq)]
struct State {
    cost: usize, 
    node: String,
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


fn dijkstra(graph: &HashMap<&str, Vec<(&str, usize)>>, start: &str) -> HashMap<String, usize> {
    let mut distances = HashMap::new();
    let mut heap = BinaryHeap::new();

    heap.push(State {
        cost:  0,
        node: start.to_string(),
    });

    while let Some(State{cost, node}) = heap.pop() {
        if distances.contains_key(&node) {
            continue;
        }

        distances.insert(node.clone(), cost);

        if let Some(neigbours) = graph.get(node.as_str()) {
            for (neighbor, weight) in neigbours {
                heap.push(State {
                    cost: cost + weight,
                    node: neighbor.to_string(),
                });
            }
        }
    }
    distances
}

                    /*
                        A - B
                        |  / \
                         C -  D
                     */

pub fn exec() {
    let graph = HashMap::from([
        ("A", vec![("B", 1), ("C", 4)]),
        ("B", vec![("C", 2), ("D", 5)]),
        ("C", vec![("D", 1)]),
        ("D", vec![]),
    ]);

    let distances = dijkstra(&graph, "A");
    println!("{:?}", distances);
}
