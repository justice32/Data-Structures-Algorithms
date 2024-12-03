use std::collections::LinkedList;


pub fn run() {
    let mut list: LinkedList<i32> = LinkedList::new();

    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    println!("{:?}", list);
    
    for value in list{
        println!("{}", value);
    }
}
