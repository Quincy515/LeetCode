struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut ans = 0;
        let mut dict = HashMap::new();
        let len = s.len();
        let v = s.chars().collect::<Vec<char>>();
        let mut j = -1;

        for i in 0..len {
            if dict.contains_key(&v[i]) {
                j = (*dict.get(&v[i]).unwrap() as i32).max(j)
            }

            dict.insert(v[i], i);

            ans = ans.max(i as i32 - j);
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::length_of_longest_substring("abcabcbb".to_string()), 3);
    }
}
