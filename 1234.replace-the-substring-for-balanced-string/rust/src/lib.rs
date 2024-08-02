use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn balanced_string(s: String) -> i32 {
        let n = s.len();
        let m = n / 4;
        let mut cnt: HashMap<char, usize> = HashMap::new();

        // 计算初始计数
        for c in s.chars() {
            *cnt.entry(c).or_insert(0) += 1;
        }

        // 检查是否已经平衡
        if cnt.len() == 4 && cnt.values().all(|&v| v == m) {
            return 0;
        }

        let mut ans = s.len() + 1;
        let mut left = 0;

        // 枚举子串右端点
        for (right, c) in s.chars().enumerate() {
            *cnt.entry(c).or_insert(0) -= 1;

            while cnt.values().all(|&v| v <= m) {
                ans = ans.min(right - left + 1);
                *cnt.entry(s.chars().nth(left).unwrap()).or_insert(0) += 1;
                left += 1; // 缩小子串
            }
        }

        ans as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::balanced_string("QWER".to_string()), 0);
        assert_eq!(Solution::balanced_string("QQWE".to_string()), 1);
        assert_eq!(Solution::balanced_string("QQQW".to_string()), 2);
        assert_eq!(Solution::balanced_string("QQQQ".to_string()), 3);
        assert_eq!(Solution::balanced_string("WQWRQQQW".to_string()), 3);
    }
}
