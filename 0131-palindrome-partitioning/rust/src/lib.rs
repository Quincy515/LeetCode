struct Solution;

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut result: Vec<Vec<String>> = vec![];
        let s = s.chars().collect::<Vec<char>>();
        Self::backtrack(&s, 0, &mut vec![], &mut result);
        result
    }

    fn backtrack(s: &[char], k: i32, path: &mut Vec<String>, result: &mut Vec<Vec<String>>) {
        if k == s.len() as i32 {
            result.push(path.clone());
            return;
        }
        for end in k..s.len() as i32 {
            if Self::ispalidrome(s, k, end) {
                path.push(s[k as usize..=end as usize].iter().collect::<String>());
                Self::backtrack(s, end + 1, path, result);
                path.pop();
            }
        }
    }
    fn ispalidrome(s: &[char], p: i32, r: i32) -> bool {
        let mut i = p;
        let mut j = r;
        while i <= j {
            if s[i as usize] != s[j as usize] {
                return false;
            }
            i += 1;
            j -= 1;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::partition("aab".to_owned()),
            vec![
                vec!["aa".to_string(), "b".to_string()],
                vec!["a".to_string(), "a".to_string(), "b".to_string()],
            ]
        );
        assert_eq!(
            Solution::partition("aaa".to_owned()),
            vec![
                vec!["aaa".to_string()],
                vec!["a".to_string(), "aa".to_string()],
                vec!["aa".to_string(), "a".to_string()],
                vec!["a".to_string(), "a".to_string(), "a".to_string()],
            ]
        );
    }
}
