use std::collections::BTreeMap;

pub fn tree_based_indexing() {
    let mut index: BTreeMap<&str, u32> = BTreeMap::new();
    index.insert("Alice",1);
    index.insert("Bob",2);
    index.insert("Charlie",3);

    if let Some(&pos) = index.get("Bob") {
        println!("Position of Bob: {}", pos);
    }
}
