use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let mut left = 0;
        let mut max = 0;
        let s_chars: Vec<char> = s.chars().collect();
        let mut cnt = HashMap::new();

        for (right, c) in s_chars.iter().enumerate() {
            *cnt.entry(c).or_insert(0) += 1;
            while right - left >= k as usize {
                // &s.chars().nth(left).unwrap()
                if let Some(count) = cnt.get_mut(&s_chars[left]) {
                    *count -= 1;
                }
                left += 1;
            }
            max = max.max(Self::count(&cnt))
        }
        max as i32
    }

    fn count(hash: &HashMap<&char, usize>) -> usize {
        let mut ret = 0;
        for (k, v) in hash {
            match k {
                'a' | 'e' | 'i' | 'o' | 'u' => ret += v,
                _ => (),
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::max_vowels("abciiidef".to_string(), 3), 3);
        assert_eq!(Solution::max_vowels("aeiou".to_string(), 2), 2);
        assert_eq!(Solution::max_vowels("leetcode".to_string(), 3), 2);
        assert_eq!(Solution::max_vowels("rhythms".to_string(), 4), 0);
        assert_eq!(Solution::max_vowels("tryhard".to_string(), 4), 1);
    }
}
