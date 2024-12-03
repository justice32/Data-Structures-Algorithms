use std::collections::BTreeMap;

pub fn b_tree_example() {
    let mut b_tree: BTreeMap<u32, &str> = BTreeMap::new();
    b_tree.insert(1, "Alice");
    b_tree.insert(2, "Bob");
    b_tree.insert(3, "Charlie");

    for (key, value) in b_tree.iter() {
        println!("{}: {}", key, value);
    }
}
