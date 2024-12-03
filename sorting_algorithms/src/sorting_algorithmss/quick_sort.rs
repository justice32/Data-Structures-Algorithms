use std::time::Instant;

pub fn quick_sort(mut arr: Vec<i32>) -> Vec<i32> {
    let start = Instant::now(); // Start timing

    fn quick_sort_helper(arr: &mut [i32]) {
        if arr.len() <= 1 {
            return;
        }

        let pivot = arr.len() / 2;
        let (mut left, right) = (0, arr.len() - 1);
        arr.swap(pivot, right);

        for i in 0..arr.len() {
            if arr[i] < arr[right] {
                arr.swap(i, left);
                left += 1;
            }
        }
        arr.swap(left, right);

        quick_sort_helper(&mut arr[..left]);
        quick_sort_helper(&mut arr[left + 1..]);
    }

    quick_sort_helper(&mut arr);

    let duration = start.elapsed(); // Stop timing
    println!(
        "Quick sort took {} seconds and {} nanoseconds",
        duration.as_secs(),
        duration.subsec_nanos()
    );

    arr
}
