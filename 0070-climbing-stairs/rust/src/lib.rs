struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let n = n as usize;
        if n <= 2 {
            return n as i32;
        }

        let mut dp = vec![0; n + 1];
        dp[1] = 1;
        dp[2] = 2;
        for i in 3..=n {
            dp[i] = dp[i - 1] + dp[i - 2];
        }
        dp[n]
    }

    pub fn climb_stairs2(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![0; n + 1];
        dp[0] = 1;
        for i in 1..=n {
            if i >= 1 {
                dp[i] += dp[i - 1];
            }
            if i >= 2 {
                dp[i] += dp[i - 2];
            }
        }
        dp[n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::climb_stairs(2), 2);
        assert_eq!(Solution::climb_stairs2(3), 3);
    }
}
