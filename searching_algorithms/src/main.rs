/*
Linear Search is the simplest searching algorithm. 
It sequentially checks each element of the array until 
the target element is found or the array is fully traversed.

Time Complexity:
Best Case:  𝑂(1)
Worst Case: 𝑂(𝑛)
Average Case: 𝑂(𝑛)
 */
pub fn linear_search(arr: &[i32], key: i32) -> Option<usize> {
    for (index, &value) in arr.iter().enumerate() {
        if value == key {
            return Some(index);
        }
    }
    None
}

/*
Binary Search is a more efficient searching algorithm, 
but it requires the array to be sorted. It divides the 
search space into halves at each step, 
eliminating half of the array from consideration. 
Best case: O(1)
Worst case: O(log n)
*/
pub fn binary_search(arr: &[i32], target: i32) -> Option<usize>{
    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        let mid = left + (right - left) / 2;
        if arr[mid] == target {
            return Some(mid);
        }  else if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    None
}

fn main() {
    let arr = vec![5, 2, 9, 1, 5, 6, 10, 3];
    let target = 6;
    match linear_search(&arr, target.clone()) {
        Some(index) => println!("Found {} at index {}", target, index),
        None => println!("{} not found in the array", target),
    }

    let arr1 = vec![1, 2, 3, 5, 5, 6, 9, 10]; 


    match binary_search(&arr1, target) {
        Some(index) => println!("found {} at index {}", target, index),
        None => println!("{} not found in the array", target),
    }
}


