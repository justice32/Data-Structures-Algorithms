pub fn isam_example() {
    let data = vec![
        ("Alice", "Record1"),
        ("Bob", "Record2"),
        ("Charlie", "Record3"),
    ];
    let index: Vec<usize> = data
    .iter()
    .enumerate()
    .map(|(i,_)|i)
    .collect();

    let pos = index[0];
    println!("Data of position {}: {:?}", pos, data[pos]);
}
