struct Solution;

impl Solution {
    pub fn count_good_substrings(s: String) -> i32 {
        let s = s.chars().collect::<Vec<char>>();
        let mut hash = std::collections::HashMap::new();
        let (mut l, mut r) = (0i32, -1i32);
        let mut result = 0;
        while r < s.len() as i32 - 1 {
            r += 1;
            let count = hash.entry(s[r as usize]).or_insert(0);
            *count += 1;
            while hash.get(&s[r as usize]) > Some(&1) || r - l + 1 > 3 {
                let tmp = hash.entry(s[l as usize]).or_insert(0);
                *tmp -= 1;
                l += 1;
            }
            if r - l + 1 == 3 {
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
        assert_eq!(Solution::count_good_substrings("xyzzaz".to_owned()), 1);
        // assert_eq!(Solution::count_good_substrings("aababcabc".to_owned()), 4);
    }
}
