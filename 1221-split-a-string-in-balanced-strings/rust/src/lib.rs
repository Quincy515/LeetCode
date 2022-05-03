struct Solution;

impl Solution {
    pub fn balanced_string_split(s: String) -> i32 {
        let (mut count, mut result) = (0, 0);
        for i in s.chars() {
            if i == 'L' {
                count += 1;
            } else {
                count -= 1;
            }
            if count == 0 {
                result += 1;
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
        assert_eq!(Solution::balanced_string_split("RLRRLLRLRL".to_owned()), 4);
        assert_eq!(Solution::balanced_string_split("RLLLLRRRLR".to_owned()), 3);
        assert_eq!(Solution::balanced_string_split("LLLLRRRR".to_owned()), 1);
        assert_eq!(Solution::balanced_string_split("RLRRRLLRLL".to_owned()), 2);
    }
}
