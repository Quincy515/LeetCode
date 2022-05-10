struct Solution;
impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let n = s.len();
        // dp[i] 表示长度为 i 的字符串时可拆分的
        let mut dp = vec![false; n + 1];
        dp[0] = true;
        for i in 1..=n {
            for word in &word_dict {
                let len = word.len();
                if i >= len && s[i - len..i] == *word && dp[i - len] {
                    dp[i] = true;
                    break;
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
        assert!(Solution::word_break(
            "leetcode".to_string(),
            vec!["leet".to_string(), "code".to_string()]
        ));
        assert!(Solution::word_break(
            "applepenapple".to_string(),
            vec!["apple".to_string(), "pen".to_string()]
        ));
        assert!(!Solution::word_break(
            "catsandog".to_string(),
            vec![
                "cats".to_string(),
                "dog".to_string(),
                "sand".to_string(),
                "and".to_string(),
                "cat".to_string()
            ]
        ));
    }
}
