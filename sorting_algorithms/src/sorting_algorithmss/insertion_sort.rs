use std::time::Instant;
pub fn insertion_sort(mut arr: Vec<i32>) -> Vec<i32> {
    let n = arr.len();
    let start = Instant::now();

    for i in 1..n{
        let key = arr[i];
        let mut j = i;
        while j > 0 && arr[j-1] > key {
            arr[j] = arr[j-1];
            j -= 1;
        }
        arr[j] = key;
    }
    let duration = start.elapsed();
    println!(" Insertion sort took {} seconds, and {} nanoseconds",  duration.as_secs(), duration.subsec_nanos());
    arr
}
