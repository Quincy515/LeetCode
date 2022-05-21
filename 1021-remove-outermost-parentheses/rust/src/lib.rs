struct Solution;

impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let mut stack = vec![];
        let mut result = String::new();
        for c in s.chars() {
            if c == '(' && stack.is_empty() {
                stack.push('(');
            } else if c == '(' && !stack.is_empty() {
                stack.push('(');
                result.push('(');
            } else if c == ')' {
                stack.pop();
                if !stack.is_empty() {
                    result.push(')');
                }
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
        assert_eq!(
            Solution::remove_outer_parentheses("(()())(())".to_string()),
            "()()()".to_string()
        );
        assert_eq!(
            Solution::remove_outer_parentheses("(()())(())(()(()))".to_string()),
            "()()()()(())".to_string()
        );
        assert_eq!(
            Solution::remove_outer_parentheses("()()".to_string()),
            "".to_string()
        );
    }
}
