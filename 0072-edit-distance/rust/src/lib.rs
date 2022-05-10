struct Solution;

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let (n, m) = (word1.len(), word2.len());
        if n == 0 {
            return m as i32;
        }
        if m == 0 {
            return n as i32;
        }
        let (w1, w2) = (word1.as_bytes(), word2.as_bytes());
        //dp[i][j] 表示 w1[0~i-1]（长度为 i 子串）和 w2[0~j-1]（长度为 j 子串）的最小编辑距离
        let mut dp = vec![vec![0; m + 1]; n + 1];
        for j in 0..=m {
            dp[0][j] = j as i32;
        }
        for i in 0..=n {
            dp[i][0] = i as i32;
        }
        for i in 1..=n {
            for j in 1..=m {
                if w1[i - 1] == w2[j - 1] {
                    dp[i][j] = (dp[i - 1][j] + 1)
                        .min(dp[i][j - 1] + 1)
                        .min(dp[i - 1][j - 1]);
                } else {
                    dp[i][j] = (dp[i - 1][j] + 1)
                        .min(dp[i][j - 1] + 1)
                        .min(dp[i - 1][j - 1] + 1)
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
            Solution::min_distance("horse".to_string(), "ros".to_string()),
            3
        );
        assert_eq!(
            Solution::min_distance("intention".to_string(), "execution".to_string()),
            5
        );
    }
}
