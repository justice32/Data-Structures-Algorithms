/// A Segment Tree implementation for range sum queries and point updates
struct SegmentTree {
    tree: Vec<i32>, // The segment tree array, which stores the segment values
    n: usize,       // The size of the original array
}

impl SegmentTree {
    /// Constructor to initialize the Segment Tree from a given array
    ///
    /// # Arguments
    /// * `arr` - A slice of integers representing the input array
    ///
    /// # Returns
    /// * A new SegmentTree object
    fn new(arr: &[i32]) -> Self {
        let n = arr.len();
        let mut tree = vec![0; 4 * n]; // Allocate enough space for the tree (4 * n is sufficient for all nodes)
        let mut seg_tree = SegmentTree { tree, n };
        seg_tree.build(arr, 0, n - 1, 0); // Build the tree starting from the root node (node 0)
        seg_tree
    }

    /// Recursively build the Segment Tree
    ///
    /// # Arguments
    /// * `arr` - The input array
    /// * `start` - The starting index of the current range
    /// * `end` - The ending index of the current range
    /// * `node` - The index of the current node in the segment tree array
    fn build(&mut self, arr: &[i32], start: usize, end: usize, node: usize) {
        if start == end {
            // If start == end, it's a leaf node, store the array value
            self.tree[node] = arr[start];
        } else {
            // Calculate the middle of the range
            let mid = (start + end) / 2;

            // Recursively build the left subtree
            self.build(arr, start, mid, 2 * node + 1);

            // Recursively build the right subtree
            self.build(arr, mid + 1, end, 2 * node + 2);

            // Store the sum of left and right subtrees in the current node
            self.tree[node] = self.tree[2 * node + 1] + self.tree[2 * node + 2];
        }
    }

    /// Query the sum of a range [qs, qe]
    ///
    /// # Arguments
    /// * `qs` - Query start index
    /// * `qe` - Query end index
    /// * `start` - The start index of the current segment
    /// * `end` - The end index of the current segment
    /// * `node` - The index of the current node in the segment tree array
    ///
    /// # Returns
    /// * The sum of elements in the range [qs, qe]
    fn query(&self, qs: usize, qe: usize, start: usize, end: usize, node: usize) -> i32 {
        // Case 1: The current segment is completely outside the query range
        if qs > end || qe < start {
            return 0; // Return 0 as it doesn't contribute to the sum
        }

        // Case 2: The current segment is completely inside the query range
        if qs <= start && qe >= end {
            return self.tree[node]; // Return the value of this node
        }

        // Case 3: The current segment partially overlaps with the query range
        let mid = (start + end) / 2;

        // Query the left and right subtrees and combine their results
        let left = self.query(qs, qe, start, mid, 2 * node + 1);
        let right = self.query(qs, qe, mid + 1, end, 2 * node + 2);

        left + right // Return the combined result
    }

    /// Update a specific index in the array to a new value
    ///
    /// # Arguments
    /// * `idx` - The index to update
    /// * `value` - The new value to set at index `idx`
    /// * `start` - The start index of the current segment
    /// * `end` - The end index of the current segment
    /// * `node` - The index of the current node in the segment tree array
    fn update(&mut self, idx: usize, value: i32, start: usize, end: usize, node: usize) {
        if start == end {
            // If it's a leaf node, directly update the value
            self.tree[node] = value;
        } else {
            let mid = (start + end) / 2;

            // If the index lies in the left subtree
            if idx <= mid {
                self.update(idx, value, start, mid, 2 * node + 1);
            } else {
                // Otherwise, it lies in the right subtree
                self.update(idx, value, mid + 1, end, 2 * node + 2);
            }

            // Update the current node after updating the child nodes
            self.tree[node] = self.tree[2 * node + 1] + self.tree[2 * node + 2];
        }
    }
}

pub fn exec() {
    // Example array
    let arr = vec![1, 3, 5, 7, 9, 11];

    // Initialize the Segment Tree
    let mut seg_tree = SegmentTree::new(&arr);

    // Query the sum of the range [1, 3] (3 + 5 + 7 = 15)
    println!("Sum of range (1, 3): {}", seg_tree.query(1, 3, 0, arr.len() - 1, 0));

    // Update index 1 to 10 (changing the array to [1, 10, 5, 7, 9, 11])
    seg_tree.update(1, 10, 0, arr.len() - 1, 0);

    // Query the sum of the range [1, 3] again (10 + 5 + 7 = 22)
    println!(
        "Sum of range (1, 3) after update: {}",
        seg_tree.query(1, 3, 0, arr.len() - 1, 0)
    );
}
