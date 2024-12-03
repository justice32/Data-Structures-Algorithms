use core::num;

fn max_sum_subarray(nums: Vec<i32> , k: usize) -> i32 {
    let mut max_sum = 0;
    let mut current_sum = nums
    .iter()
    .take(k)
    .sum::<i32>();

    max_sum = current_sum;

    for i in k..nums.len() {
        current_sum += nums[i] - nums[i - k];
        max_sum = max_sum.max(current_sum);
    }
    max_sum
}

pub fn exec() {
    let nums = vec![1,2,3,4,5];
    let k = 3;
    let max_sum = max_sum_subarray(nums, k);
    println!("Max sum of subarray is {}: {}", k, max_sum);
}
