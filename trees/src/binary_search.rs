#[derive(Debug)]

pub struct BST_Node {
    pub value: i32,
    pub left: Option<Box<BST_Node>>,
    pub right: Option<Box<BST_Node>>,
}

impl BST_Node{ 
    pub fn new(value: i32) -> Self {
        BST_Node {
            value,
            left: None,
            right: None
        }
    }

    pub fn insert(&mut self, value: i32) {
        if value < self.value {
            if let Some(ref mut left) = self.left {
                left.insert(value);
            } else {
                self.left = Some(Box::new(BST_Node::new(value)));
            }
        } else {
            if let Some(ref mut right) = self.right {
                right.insert(value);
            } else {
                self.right = Some(Box::new(BST_Node::new(value)));
            }
        }
    }
}


pub fn example() {

    let mut root = BST_Node::new(10);
    
    root.insert(5);
    root.insert(10);
    root.insert(15);
    println!("{:?}", root);
}
