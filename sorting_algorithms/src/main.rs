mod sorting_algorithmss;
use crate::sorting_algorithmss::bubble_sort::bubble_sort;
use crate::sorting_algorithmss::insertion_sort::insertion_sort;
use crate::sorting_algorithmss::selection_sort::selection_sort;
use crate::sorting_algorithmss::merge_sort::merge_sort;
use crate::sorting_algorithmss::quick_sort::quick_sort;
use crate::sorting_algorithmss::heap_sort::heap_sort;
fn main() {
    let arr = vec![5, 2, 9, 1, 5, 6, 10, 3];
    /* BUBBLE SORT
    Checks 2 elements at the same time, and change in case b<a
    Time complexity: 
    Best case: O(N)
    Worst case: O(N^2)
     */
    println!("Bubble sort!: {:?}", bubble_sort(arr.clone()));


    /*
    INSERTION SORT place key element in the correct place, it took 1 element, and check with where key element(K)
    K-1 < K < K+1
    Best case: O(N) - already sorted
    Worst case: O(N^2)
     */
    println!("Insertion sort!: {:?}", insertion_sort(arr.clone()));


    /*
    SELECTION SORT 
    It divides the array into two parts (sorted and unsorted) 
    and selects the smallest element from the unsorted part to add to the sorted part.
    Best case: O(N^2)
    Worst case: O(N^2)
     */
    println!("Selection sort!: {:?}", selection_sort(arr.clone()));



    /*
    Merge sort
    Merge sort is a divide-and-conquer algorithm that splits the array into halves, sorts each half recursively, and then merges the two sorted halves.
    Best case: O(n log n)
    Worst case: O(n log n)
     */
    println!("Merge sort!: {:?}", merge_sort(arr.clone()));


    /*
    Quick sort  is a divide-and-conquer algorithm that partitions the array into two sub-arrays and recursively sorts them
    Best case: O(n log n)
    Worst case: O(n log n)
    */
    println!("Quick sort!: {:?}", quick_sort(arr.clone()));

    /*
    Heap sort
    Heap sort builds a max-heap and repeatedly extracts the largest element to build a sorted array. 
    Best case: O(n log n)
    Worst case: O(n log n)
    */
    
    println!("Heap sort!: {:?}", heap_sort(arr.clone()));

}
