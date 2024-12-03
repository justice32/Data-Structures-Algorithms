struct FenwickTree {
    tree: Vec<i32>,
}

impl FenwickTree {
    fn new(size: usize) -> Self {
        FenwickTree {
            tree: vec![0; size + 1], // 7 
        }
    }
    fn update(&mut self, index: usize, value: i32) {
        let mut i = index as isize;
        while i < self.tree.len() as isize {
            self.tree[i as usize] += value;
            i += i & -i;
        }
    }

    fn query(&self, index: usize) -> i32 {
        let mut sum = 0;
        let mut i = index as isize;
        while i > 0 {
            sum += self.tree[i as usize];
            i -= i & -i;
        }
        sum
    }
}

pub fn exec() {
    let mut fenwick = FenwickTree::new(6);
    fenwick.update(1, 1);
    fenwick.update(2, 3);
    fenwick.update(3, 5);
    println!("Prefix sum up to index 3: {}", fenwick.query(3)); // 9
}
