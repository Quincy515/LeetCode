struct Solution;

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let n = coins.len();
        // 第 i 个硬币决策完之后，凑足金额 j 需要的最少硬币数 dp[i][j]
        let mut dp = vec![vec![i32::MAX; amount as usize + 1]; n];
        let mut c = 0;
        while c <= amount / coins[0] {
            dp[0][(c * coins[0]) as usize] = c;
            c += 1;
        }
        for i in 1..n {
            for j in 0..=amount as usize {
                let k = j / coins[i] as usize;
                for c in 0..=k {
                    if dp[i - 1][j - c * coins[i] as usize] != i32::MAX
                        && dp[i - 1][j - c * coins[i] as usize] + (c as i32) < dp[i][j]
                    {
                        dp[i][j] = dp[i - 1][j - c * coins[i] as usize] + c as i32;
                    }
                }
            }
        }
        if dp[n - 1][amount as usize] == i32::MAX {
            return -1;
        }
        dp[n - 1][amount as usize]
    }
    pub fn coin_change2(coins: Vec<i32>, amount: i32) -> i32 {
        let k = coins.len();
        let mut dp = vec![i32::MAX; amount as usize + 1];
        dp[0] = 0;
        for i in 1..=amount {
            for j in 0..k {
                if i - coins[j] >= 0
                    && dp[(i - coins[j]) as usize] != i32::MAX
                    && dp[(i - coins[j]) as usize] + 1 < dp[i as usize]
                {
                    dp[i as usize] = dp[(i - coins[j]) as usize] + 1;
                }
            }
        }
        if dp[amount as usize] == i32::MAX {
            return -1;
        }
        dp[amount as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::coin_change(vec![1, 2, 5], 11), 3);
        assert_eq!(Solution::coin_change2(vec![2], 3), -1);
        assert_eq!(Solution::coin_change2(vec![1], 0), 0);
    }
}
