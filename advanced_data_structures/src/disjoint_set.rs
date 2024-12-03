use std::collections::btree_set::Difference;

struct DisjointSet { 
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl DisjointSet {
    fn new(size: usize) -> Self { // 5 
        DisjointSet {
            parent: (0..size).collect(), // 5
            rank: vec![0;size], //5 
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]); // compression ?         
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x != root_y {
            if self.rank[root_x] > self.rank[root_y] {
                self.parent[root_y] = root_x;                
            }
            else if self.rank[root_x] < self.rank[root_y] {
                self.parent[root_x] = root_y;
            } else {
                self.parent[root_y] = root_x;
                self.rank[root_x] += 1;
            }
        }
    }
}


pub fn exec() {
    let mut ds = DisjointSet::new(5);
    ds.union(0, 1);
    ds.union(1, 2);
    println!("Are 0 and 2 connected? {}", ds.find(0) == ds.find(2));
}
