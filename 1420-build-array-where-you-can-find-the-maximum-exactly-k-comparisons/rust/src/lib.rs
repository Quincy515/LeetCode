struct Solution;

impl Solution {
    pub fn num_of_arrays(n: i32, m: i32, k: i32) -> i32 {
        let mood = (10e9 + 7.0) as i32;
        let (n, m, k) = (n as usize, m as usize, k as usize);
        let mut dp = vec![vec![vec![0; 52]; 102]; 105];
        for j in 1..m + 1 {
            dp[1][j][1] = 1;
        }
        for i in 1..n + 1 {
            for j in 1..m + 1 {
                for t in 1..k + 1 {
                    for _ in 1..j + 1 {
                        dp[i + 1][j][t] += dp[i][j][t];
                        if dp[i + 1][j][t] >= mood {
                            dp[i + 1][j][t] -= mood;
                        }
                    }
                    for jj in j + 1..m + 1 {
                        dp[i + 1][jj][t + 1] += dp[i][j][t];
                        if dp[i + 1][jj][t + 1] >= mood {
                            dp[i + 1][jj][t + 1] -= mood;
                        }
                    }
                }
            }
        }

        let mut result = 0;
        for i in 1..m + 1 {
            result += dp[n][i][k];
            if result >= mood {
                result -= mood;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(Solution::num_of_arrays(2, 3, 1), 6);
        assert_eq!(Solution::num_of_arrays(5, 2, 3), 0);
        assert_eq!(Solution::num_of_arrays(9, 1, 1), 1);
        assert_eq!(Solution::num_of_arrays(50, 100, 25), 34549172);
        assert_eq!(Solution::num_of_arrays(37, 17, 7), 418930126);
    }
}
