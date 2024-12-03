mod binary;
mod binary_search;
mod avl_trees;
use std::collections::BTreeMap;
mod tree_traversal;
mod search_algorithms_tree;
fn main() { 

    /*
    A binary tree is a tree data structure in which each
    node has at most two children: a left child and a right child. */
    //println!("Binary tree");
    //binary::example();


    /*
    A binary search tree (BST) is a binary tree where the 
    value of each node is greater than or equal to the values
    in its left subtree and less than or equal to the values
    in its right subtree. */
    //println!("Binary Search tree");
    //binary_search::example();


    /*
    An AVL tree is a self-balancing binary search tree, 
    where the difference between the heights of left and right 
    subtrees cannot be more than one for all nodes. */
    //println!("AVL trees");
    //avl_trees::example();

    /* 
    let mut btree = BTreeMap::new();
    btree.insert(1, "One");
    btree.insert(3, "Three");
    btree.insert(2, "Two");
    println!("{:?}", btree);
*/


    //tree_traversal::example();

    search_algorithms_tree::example();
}
