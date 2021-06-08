impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
    let mut dp = vec![1,2];
    let n = n as usize;
    for i in 2..n {
        dp.push(
            dp[i - 2] + dp[i - 1]
        )
    } 
    dp[n - 1]
    }
}