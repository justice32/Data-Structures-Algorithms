#[derive(Debug)]

struct AVLNode{
    value: i32, 
    height: i32, 
    left: Option<Box<AVLNode>>,
    right: Option<Box<AVLNode>>,
}

impl AVLNode {
    fn new(value: i32) -> Self {
        AVLNode {
            value, 
            height: 1,
            left: None,
            right: None,    
        }
    }


    fn height(node: &Option<Box<AVLNode>>) -> i32 {
        match node {
            Some(node) => node.height,
            None => 0,
        }
    }

    fn balance_factor(&self) -> i32 {
        Self::height(&self.left) - Self::height(&self.right)
    }
}

pub fn example() {
    let root = AVLNode::new(10);
    
    println!("{:?}", root);
}
