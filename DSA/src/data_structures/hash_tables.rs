use std::collections::HashMap;

pub fn run() {
    let mut scores: HashMap<&str, i32> = HashMap::new();
    scores.insert("Alice", 50);
    scores.insert("Bob", 30);

    for (key, value) in scores.iter() {
        println!("{}: {}", key, value);
    }
}
