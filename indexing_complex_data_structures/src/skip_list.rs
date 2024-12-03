use skiplist::SkipMap;

pub fn skip_list_example() {
    let mut skip_list = SkipMap::new();
    skip_list.insert(1, "Alice");
    skip_list.insert(2, "Bob");
    skip_list.insert(3, "Charlie");

    if let Some(node) = skip_list.get(&1) {
        println!("Found: {}", node);
    }
}
