struct Solution;

impl Solution {
    pub fn di_string_match(s: String) -> Vec<i32> {
        let mut res = vec![0i32; s.len() + 1];
        let (mut left, mut right) = (0, s.len() as i32);
        for (i, c) in s.chars().enumerate() {
            if c == 'I' {
                res[i] = left;
                left += 1;
            } else {
                res[i] = right;
                right -= 1;
            }
        }
        res[s.len()] = left;
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(
            Solution::di_string_match("IDID".to_owned()),
            vec![0, 4, 1, 3, 2]
        );
        assert_eq!(
            Solution::di_string_match("III".to_owned()),
            vec![0, 1, 2, 3]
        );
        assert_eq!(
            Solution::di_string_match("DDI".to_owned()),
            vec![3, 2, 0, 1]
        );
    }
}
