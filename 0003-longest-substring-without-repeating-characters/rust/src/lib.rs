use std::collections::HashMap;
use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn length_of_longest_substring_v1(s: String) -> i32 {
        let mut ans = 0;
        let mut left = 0;
        let mut window = HashSet::new(); // 维护从下标 left 到下标 right 的字符
        let s_chars: Vec<char> = s.chars().collect();

        for (right, c) in s_chars.iter().enumerate() {
            // 如果窗口内已经包含 c，那么再加入一个 c 会导致窗口内有重复元素
            // 所以要在加入 c 之前，先移出窗口内的 c
            while window.contains(c) {
                // 窗口内有 c
                window.remove(&s_chars[left]);
                left += 1; // 缩小窗口
            }
            window.insert(*c); // 加入 c
            ans = ans.max(right - left + 1); // 更新窗口长度最大值
        }
        ans as i32
    }

    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut ans = 0;
        let mut left = 0;
        let mut char_count = HashMap::new(); // 记录每个字符出现的次数

        for (right, c) in s.chars().enumerate() {
            // 增加字符出现的次数
            *char_count.entry(c).or_insert(0) += 1;

            // 如果有字符的出现次数大于1，就需要收缩窗口
            while *char_count.get(&c).unwrap() > 1 {
                if let Some(count) = char_count.get_mut(&s.chars().nth(left).unwrap()) {
                    *count -= 1;
                }
                left += 1;
            }

            // 更新最长子串的长度
            ans = ans.max(right - left + 1);
        }

        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_string()),
            3
        );
        assert_eq!(
            Solution::length_of_longest_substring("bbbbb".to_string()),
            1
        );
        assert_eq!(
            Solution::length_of_longest_substring("pwwkew".to_string()),
            3
        );
    }
}
