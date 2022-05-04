struct Solution;

impl Solution {
    pub fn reverse_only_letters(s: String) -> String {
        let (mut left, mut right) = (0, s.len() - 1);
        let mut s = s.chars().collect::<Vec<char>>();
        for _ in 0..s.len() {
            if left >= right {
                break;
            }
            if s[left].is_ascii_alphabetic() && s[right].is_ascii_alphabetic() {
                s.swap(left, right);
                left += 1;
                right -= 1;
            }
            if !s[left].is_ascii_alphabetic() {
                left += 1;
            }
            if !s[right].is_ascii_alphabetic() {
                right -= 1;
            }
        }

        s.iter().collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(Solution::reverse_only_letters("ab-cd".to_string()), "dc-ba");
        assert_eq!(
            Solution::reverse_only_letters("a-bC-dEf-ghIj".to_string()),
            "j-Ih-gfE-dCba"
        );
        assert_eq!(
            Solution::reverse_only_letters("Test1ng-Leet=code-Q!".to_string()),
            "Qedo1ct-eeLg=ntse-T!"
        );
        assert_eq!(Solution::reverse_only_letters("mv']4".to_string()), "vm']4");
    }
}
