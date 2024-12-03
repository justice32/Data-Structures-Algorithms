use crate::binary::Node;
use std::collections::VecDeque;

pub fn bfs(root: Option<Box<Node<i32>>>){
    let mut queue = VecDeque::new();
    if let Some(node) = root {
        queue.push_back(node);
    }
    while let Some(node) = queue.pop_front() {
        println!("{}", node.value);
        if let Some(left) = node.left {
            queue.push_back(left);
        }
        if let Some(right) = node.right {
            queue.push_back(right);
        }
    }
}

pub fn dfs(node: &Option<Box<Node<i32>>>) {
    if let Some(node) = node {
        println!("{}", node.value);
        dfs(&node.left);
        dfs(&node.right)
    }
}

pub fn example() {
    let mut root = Node::new(1);
    root.left = Some(Box::new(Node::new(2)));
    root.right = Some(Box::new(Node::new(3)));
    println!("BFS:");
    bfs(Some(Box::new(root.clone())));

    println!("DFS:");
    dfs(&Some(Box::new(root)));
}
