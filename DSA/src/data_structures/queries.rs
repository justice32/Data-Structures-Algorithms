use std::collections::VecDeque;

pub fn run() {
    let mut queue: VecDeque<i32> = VecDeque::new();
    queue.push_back(10);
    queue.push_back(20);
    queue.push_back(30);
    println!("{:?}", queue);

    while let Some(number) = queue.pop_front() {
        println!("{}", number);
    }
}
