use std::time::Instant;

pub fn merge_sort(mut arr: Vec<i32>) -> Vec<i32> {
    // Helper function to merge two halves of the array
    fn merge(arr: &mut [i32], left: usize, mid: usize, right: usize) {
        let mut temp = vec![];
        let mut i = left;
        let mut j = mid + 1;

        // Merge elements from both halves into the temp vector
        while i <= mid && j <= right {
            if arr[i] <= arr[j] {
                temp.push(arr[i]);
                i += 1;
            } else {
                temp.push(arr[j]);
                j += 1;
            }
        }

        // Add remaining elements from the left half
        while i <= mid {
            temp.push(arr[i]);
            i += 1;
        }

        // Add remaining elements from the right half
        while j <= right {
            temp.push(arr[j]);
            j += 1;
        }

        // Copy the sorted temp vector back to the original array
        arr[left..=right].copy_from_slice(&temp);
    }

    // Recursive function to perform merge sort
    fn merge_sort_helper(arr: &mut [i32], left: usize, right: usize) {
        if left < right {
            let mid = (left + right) / 2;
            merge_sort_helper(arr, left, mid);
            merge_sort_helper(arr, mid + 1, right);
            merge(arr, left, mid, right);
        }
    }

    // Start timing the entire merge sort
    let start = Instant::now();

    let len = arr.len();
    if len > 1 {
        merge_sort_helper(&mut arr, 0, len - 1);
    }

    // End timing and print the duration
    let duration = start.elapsed();
    println!(
        "Merge sort took {} seconds and {} nanoseconds",
        duration.as_secs(),
        duration.subsec_nanos()
    );

    arr
}
