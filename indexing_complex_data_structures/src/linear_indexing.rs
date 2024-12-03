pub fn linear_indexing() {
    let data = vec![("Alice", 1), ("Bob", 2), ("Charlie", 3)];
    let index = data
    .iter()
    .position(|&(name,_)|name== "Alice");
    println!("index of Bob: {:?}", index);
}
