use std::time::Instant;

pub fn heap_sort(mut arr: Vec<i32>) -> Vec<i32> {
    // Start timing the sorting process
    let start = Instant::now();

    // Function to heapify a subtree with root at index `i`
    fn heapify(arr: &mut [i32], n: usize, i: usize) {
        let mut largest = i; // Assume root is the largest
        let left = 2 * i + 1; // Left child
        let right = 2 * i + 2; // Right child

        // If left child is larger than root
        if left < n && arr[left] > arr[largest] {
            largest = left;
        }

        // If right child is larger than the largest so far
        if right < n && arr[right] > arr[largest] {
            largest = right;
        }

        // If largest is not root, swap and recursively heapify
        if largest != i {
            arr.swap(i, largest);
            heapify(arr, n, largest);
        }
    }

    let n = arr.len();

    // Step 1: Build a max-heap
    for i in (0..n / 2).rev() {
        heapify(&mut arr, n, i);
    }

    // Step 2: Extract elements from the heap
    for i in (1..n).rev() {
        arr.swap(0, i); // Move the current root to the end
        heapify(&mut arr, i, 0); // Call heapify on the reduced heap
    }

    // Stop timing after the sorting is complete
    let duration = start.elapsed();
    println!(
        "Heap sort took {} seconds and {} nanoseconds",
        duration.as_secs(),
        duration.subsec_nanos()
    );

    // Return the sorted array
    arr
}
