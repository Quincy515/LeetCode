#![allow(dead_code)]
struct Solution;


impl Solution {
  pub fn ways_to_step(n: i32) -> i32 {
    if n == 1 { return 1; }
    if n == 2 { return 2; }
    if n == 3 { return 4; }
    let mut dp = vec![0;n as usize+1];
    dp[1] =1;
    dp[2] = 2;
    dp[3] = 4;
    for i in 4..=n {
      dp[i as usize] = ((dp[i as usize -1] + dp[i as usize -2]) % 1000000007 + dp[i as usize -3])%1000000007;
    }
    return dp[n as usize];
  }
}