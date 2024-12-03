fn backtrack_subset(nums: &[i32], 
                    path: &mut Vec<i32>, 
                    res: &mut Vec<Vec<i32>>, 
                    index: usize) {
    res.push(path.clone());
    for i in index..nums.len() {
        path.push(nums[i]);
        backtrack_subset(nums, path, res, i + 1);
        path.pop();
    }
}

pub fn exec() {
    let nums = vec![1,2,3];
    let mut res = Vec::new();
    backtrack_subset(&nums, &mut Vec::new(), &mut res, 0);
    println!("Subsets: {:?}", res);
}
