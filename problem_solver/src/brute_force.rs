pub fn brute_force_max_subarray(arr: &[i32]) -> i32 {
    let mut max_sum = i32::MIN;
    for i in 0..arr.len() {
        for j in i..arr.len() {
            let sum: i32 = arr[i..=j].iter().sum();
            println!("iter sum: {}", sum);
            max_sum = max_sum.max(sum);
        }
    }
    max_sum
}

pub fn exec() {   /*|  --->              */     
    let arr = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    /*
    2 -> 1 + -3 + 4 + -4 + 2 + 1 + - 5 + 4
    ..
    ..
    2 -> 1 + -5 + 4
     */
    println!("Max subarray sum: {}", brute_force_max_subarray(&arr));
}
