#[derive(Debug, Clone)]
pub struct Node<T> {
    pub value: T, 
    pub left: Option<Box<Node<T>>>,
    pub right: Option<Box<Node<T>>>,
}


impl <T> Node<T> {
    pub fn new(value: T) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }
}


pub fn example() {
    let mut root =  Node::new(1);
    root.left = Some(Box::new(Node::new(2)));
    root.right = Some(Box::new(Node::new(3)));
    println!("{:?}", root);
}
