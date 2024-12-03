use crate::binary::Node;

pub fn in_order_traversal(node: &Option<Box<Node<i32>>>) {
    if let Some(node) = node {
        in_order_traversal(&node.left);
        println!("{}", node.value);
        in_order_traversal(&node.right);
    }
}

pub fn pre_order_traversal(node: &Option<Box<Node<i32>>>) {
    if let Some(node) = node {
        println!("{}", node.value);
        pre_order_traversal(&node.left);
        pre_order_traversal(&node.right);
    }
}

pub fn post_order_trarsal(node: &Option<Box<Node<i32>>>) {
    if let Some(node) = node {
        post_order_trarsal(&node.left);
        post_order_trarsal(&node.right);
    }
}


pub fn example() {
    let mut root = Node::new(1);
    root.left = Some(Box::new(Node::new(2)));
    root.right = Some(Box::new(Node::new(3)));

    println!("In Order traversal");
    in_order_traversal(&Some(Box::new(root)));
}
