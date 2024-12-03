fn merge_sort(nums: &mut [i32]) {
    if nums.len() <= 1 {
        return;
    }

    let mid = nums.len() /2 ;
    let mut left = nums[0..mid].to_vec();
    let mut right = nums[mid..].to_vec();

    merge_sort(&mut left);
    merge_sort(&mut right);

    merge(nums, &left, &right);
}


fn merge(result: &mut [i32], 
         left: &[i32],
         right: &[i32]) {

    let mut i = 0;
    let mut j = 0;

    for k in 0..result.len() {
        
        if i >= left.len() {
            result[k] = right[j];
            j += 1;
        } else if j >= right.len() || left[i] <= right[j] {
            result[k] = left[i];
            i +=1;
        } else {
            result[k] = right[j];
            j += 1;
        }
    }
}


pub fn exec() {
    let mut nums = vec![3,5,2,9,1];
    merge_sort(&mut nums);
    println!("{:?}", nums);
}
