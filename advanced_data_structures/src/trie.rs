/*
A Trie is a tree-like data structure used to efficiently store and search strings, commonly used for autocomplete and dictionary applications.
Explanation:
Each node represents a character. Paths from the root to leaves represent words. 
Searching for a string is 𝑂(𝐿) where 𝐿 is the length of the string.
*/
use std::{collections::HashMap, hash::Hash};
struct TriNode {
    children: HashMap<char, TriNode>,
    is_end_of_word: bool,
}
impl TriNode {
    fn new() -> Self {
        TriNode{
            children: HashMap::new(),
            is_end_of_word: false,
        }
    }
}
struct Trie {
    root: TriNode,
}
impl Trie {
    fn new() -> Self {
        Trie {
            root: TriNode::new(),
        }
    }
    fn insert(&mut self, word: &str) {
        let mut node = &mut self.root;
        for ch in word.chars() {
            node = node.children.entry(ch).or_insert(TriNode::new());
        }
        node.is_end_of_word = true;
    }
        fn search(&self, word: &str) -> bool { 
            let mut node = &self.root;
            for ch in word.chars() {
                if let Some(next_node) = node.children.get(&ch) {
                    node = next_node;
                } else {
                    return false;
                }
            }
            node.is_end_of_word
        }
}

pub fn exec() {
    let mut trie = Trie::new();
    trie.insert("apple");
    trie.insert("app");

    println!("Search 'apple': {}", trie.search("apple"));
    println!("Search 'app': {}", trie.search("app"));
    println!("Search 'appl': {}", trie.search("appl"));
}

