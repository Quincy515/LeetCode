struct Solution;

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let (n, m) = (text1.len(), text2.len());
        let (t1, t2) = (text1.as_bytes(), text2.as_bytes());

        // dp[i][j] 表示 text1[0~i-1] （长度为 i 子串）和 text2[0~j-1]（长度 j 的字串）的最长公共子序列（LCS）
        let mut dp = vec![vec![0; m + 1]; n + 1];
        for j in 0..=m {
            dp[0][j] = 0;
        }
        for i in 0..=n {
            dp[i][0] = 0;
        }
        for i in 1..=n {
            for j in 1..=m {
                if t1[i - 1] == t2[j - 1] {
                    dp[i][j] = (dp[i - 1][j - 1] + 1).max(dp[i - 1][j]).max(dp[i][j - 1])
                } else {
                    dp[i][j] = dp[i - 1][j - 1].max(dp[i - 1][j]).max(dp[i][j - 1]);
                }
            }
        }

        dp[n][m]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::longest_common_subsequence("abcde".to_string(), "ace".to_string()),
            3
        );
        assert_eq!(
            Solution::longest_common_subsequence("abc".to_string(), "abc".to_string()),
            3
        );
        assert_eq!(
            Solution::longest_common_subsequence("abc".to_string(), "def".to_string()),
            0
        );
    }
}
