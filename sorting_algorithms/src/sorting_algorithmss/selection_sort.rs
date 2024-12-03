use std::time::Instant;

pub fn selection_sort(mut arr: Vec<i32>) -> Vec<i32> {
    let n = arr.len();
    let start = Instant::now();

    for i in 0..n{
        let mut min_index = i;
        for j in i+1..n {
            if arr[j] < arr[min_index]{
                min_index = j;
            }
        }
        arr.swap(i, min_index);
    }
    let duration = start.elapsed();
    println!("Selection sort took {} seconds, and {} nanoseconds",  duration.as_secs(), duration.subsec_nanos());
    arr
}
