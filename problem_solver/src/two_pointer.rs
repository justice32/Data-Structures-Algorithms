fn two_sum_sorted(nums: Vec<i32>, target: i32) -> Vec<(i32, i32)> {
    let mut pairs = vec![];
    let (mut left, mut right) = (0, nums.len() - 1 );

    while left < right { 
        let sum = nums[left] + nums[right];
        if sum == target {
            pairs.push((nums[left], nums[right]));
            left += 1;
            right -= 1;
        } else if sum < target {
            left += 1;
        } else {
            right -= 1;
        }
    }
    pairs
}

pub fn exec() {
    let nums = vec![1,2,3,4,5,6];
    let target = 6;
    let pairs = two_sum_sorted(nums, target);
    println!("{:?}", pairs);
}
