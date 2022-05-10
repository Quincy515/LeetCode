struct Solution;

impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let (s, p) = (s.as_bytes(), p.as_bytes());
        let (n, m) = (s.len(), p.len());
        if m > n {
            return vec![];
        }
        let mut needs = vec![0; 26];
        for i in 0..m {
            needs[(p[i] - b'a') as usize] += 1;
        }
        let mut matched = vec![0; 26];

        let (mut startp, mut endp) = (0, 0);
        let mut result = vec![];
        while endp < m {
            matched[(s[endp] - b'a') as usize] += 1;
            endp += 1;
        }
        if Self::same(&needs, &matched) {
            result.push(startp as i32);
        }
        while endp < n && startp < n {
            matched[(s[startp] - b'a') as usize] -= 1;
            matched[(s[endp] - b'a') as usize] += 1;
            startp += 1;
            endp += 1;
            if Self::same(&needs, &matched) {
                result.push(startp as i32);
            }
        }
        result
    }
    fn same(needs: &[i32], matched: &[i32]) -> bool {
        for i in 0..needs.len() {
            if needs[i] != matched[i] {
                return false;
            }
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
            Solution::find_anagrams("cbaebabacd".to_string(), "abc".to_string()),
            vec![0, 6]
        );
        assert_eq!(
            Solution::find_anagrams("abab".to_string(), "ab".to_string()),
            vec![0, 1, 2]
        );
    }
}
