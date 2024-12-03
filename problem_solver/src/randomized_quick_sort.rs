use rand::Rng;

fn randomized_patrition(nums: &mut [i32],
                        low: usize, 
                        high: usize) -> usize {

    let mut rng = rand::thread_rng();
    let pivot_index = rng.gen_range(low..=high);
    nums.swap(pivot_index, high);
    let mut pivot = nums[high];
    let mut i = low;
    for j in low..high {
        if nums[j] <= pivot {
        nums.swap(i,j);
        i += 1;
        }
    }
    nums.swap(i, high);
    i
}

fn randomized_quick_sort(nums: &mut [i32], low: usize, high: usize) {
    if low < high {
        let pi = randomized_patrition(nums, low, high);
        if pi > 0 {
            randomized_quick_sort(nums, low, pi - 1);
        }
        randomized_quick_sort(nums, pi + 1, high);
    }
}
        
pub fn exec() {
    let mut nums = vec![3,5,2,9,1];
    let len = nums.len();
    randomized_quick_sort(&mut nums, 0, len -1 );
    println!("{:?}", nums);
}
