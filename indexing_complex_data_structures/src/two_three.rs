struct Node{ 
    keys: Vec<i32>,
    children: Vec<Box<Node>>,
}

impl Node {
    fn new() -> Self {
        Node {
            keys: Vec::new(),
            children: Vec::new(),
        }
    }
    fn insert(&mut self, key: i32) {
        self.keys.push(key);
        self.keys.sort();
    }
    fn remove(&mut self, key: i32) -> Option<i32> {
        if let Some(pos) = self.keys.iter().position(|&k| k == key) {
            Some(self.keys.remove(pos))
        } else {
            None
        }
    }
}

pub fn two_three_tree_example() {
    let mut root = Node::new();
    root.insert(10);
    root.insert(20);
    root.insert(5);
    println!("Root keys: {:?}", root.keys);

     // Remove key 10
     if let Some(removed_key) = root.remove(10) {
        println!("Removed key: {}", removed_key);
    } else {
        println!("Key not found");
    }

    println!("Root keys after removal: {:?}", root.keys);
}
