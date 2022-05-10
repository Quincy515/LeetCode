use std::collections::HashMap;
struct Solution;
impl Solution {
    pub fn min_steps(s: String, t: String) -> i32 {
        let (mut hashs, mut hasht) = (HashMap::new(), HashMap::new());
        let (s, t) = (
            s.chars().collect::<Vec<char>>(),
            t.chars().collect::<Vec<char>>(),
        );
        let mut result = 0;
        for i in 0..s.len() {
            *hashs.entry(s[i]).or_insert(0) += 1;
            *hasht.entry(t[i]).or_insert(0) += 1;
        }
        for i in 'a'..='z' {
            let x = *hashs.entry(i).or_insert(0) - *hasht.entry(i).or_insert(0);
            if x > 0 {
                result += x;
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
        assert_eq!(Solution::min_steps("bab".to_owned(), "aba".to_owned()), 1);
    }
}
