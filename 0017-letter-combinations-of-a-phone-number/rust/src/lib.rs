impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut result: Vec<String> = vec![];
        if digits.is_empty() {
            return result;
        }
        let mut mappings = vec!["".to_string(); 10];
        mappings[2] = "abc".to_string();
        mappings[3] = "def".to_string();
        mappings[4] = "ghi".to_string();
        mappings[5] = "jkl".to_string();
        mappings[6] = "mno".to_string();
        mappings[7] = "pqrs".to_string();
        mappings[8] = "tuv".to_string();
        mappings[9] = "wxyz".to_string();
        let mut path = vec![' '; digits.len()];
        let digits: Vec<char> = digits.chars().collect();
        Self::backtrack(&mappings, &digits, 0, &mut path, &mut result);
        result
    }
    fn backtrack(
        mappings: &Vec<String>,
        digits: &Vec<char>,
        k: i32,
        path: &mut Vec<char>,
        result: &mut Vec<String>,
    ) {
        if k as usize == digits.len() {
            result.push(path.iter().collect::<String>());
            return;
        }
        let mapping = mappings[digits[k as usize].to_digit(10).unwrap() as usize].clone();
        for c in mapping.chars() {
            path[k as usize] = c;
            Self::backtrack(mappings, digits, k + 1, path, result);
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::letter_combinations("23".to_string()),
            vec![
                "ad".to_string(),
                "ae".to_string(),
                "af".to_string(),
                "bd".to_string(),
                "be".to_string(),
                "bf".to_string(),
                "cd".to_string(),
                "ce".to_string(),
                "cf".to_string()
            ]
        );
        let empty: Vec<String> = vec![];
        assert_eq!(Solution::letter_combinations(String::new()), empty);
        assert_eq!(
            Solution::letter_combinations("2".to_string()),
            vec!["a".to_string(), "b".to_string(), "c".to_string()]
        );
    }
}
