use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn can_construct(s: String, k: i32) -> bool {
        if s.len() < k as usize {
            return false;
        }
        if s.len() == k as usize {
            return true;
        }
        let mut m = HashMap::new();
        for i in s.chars() {
            let count = m.entry(i).or_insert(0);
            *count += 1;
        }

        let mut odd = 0;
        for v in m.values() {
            if v % 2 == 1 {
                odd += 1;
            }
        }

        if k < odd {
            return false;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(!Solution::can_construct("cr".to_owned(), 7));
    }
}
