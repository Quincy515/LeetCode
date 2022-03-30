struct Solution;

impl Solution {
    // 递归转非递归的写法，类比上台阶
    pub fn cutting_rope(n: i32) -> i32 {
        let n = n as usize;
        if n == 1 {
            return 1;
        }
        if n == 2 {
            return 1;
        }
        if n == 3 {
            return 2;
        }
        // dp[i]表示长度为 i 的最大乘积
        let mut dp: Vec<i32> = vec![0; n + 1];
        dp[0] = 1;
        for i in 1..=n {
            for j in 1..=i {
                if dp[i] < j as i32 * dp[i - j] {
                    dp[i] = j as i32 * dp[i - j];
                }
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
        assert_eq!(Solution::cutting_rope(2), 1);
        assert_eq!(Solution::cutting_rope(10), 36);
    }
}
