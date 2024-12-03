use std::time::Instant;

pub fn bubble_sort(mut arr: Vec<i32>) -> Vec<i32> {
    let start = Instant::now();

    let len = arr.len();
    
    for i in 0..len {
        for j in 0..len - i - 1 {
            if arr[j] > arr[j+1]{
                arr.swap(j, j+1);
            }
        }
    }
    let duration = start.elapsed();
    println!("Bubble sort took {} seconds, and {} nanoseconds",  duration.as_secs(), duration.subsec_nanos());
    arr
}
