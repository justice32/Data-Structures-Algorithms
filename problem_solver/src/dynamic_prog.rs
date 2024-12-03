fn fibonacci_dp(n: usize) -> i32 {
    let mut dp = vec![0; n+1];
    dp[1] = 1;

    for i in 2..=n {
        dp[i] = dp[i-1] + dp[i-2];
    }
    dp[n]
}


pub fn exec() {
    let n = 10;
    let result = fibonacci_dp(n);
    println!("Fibonacci {} = {}", n, result);
}
