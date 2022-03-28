struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result: Vec<String> = vec![];
        Self::backtrack(n, 0, 0, 0, &mut vec![' '; 2 * n as usize], &mut result);
        result
    }

    fn backtrack(
        n: i32,
        left_used: i32,
        right_used: i32,
        k: i32,
        path: &mut Vec<char>,
        result: &mut Vec<String>,
    ) {
        if k == 2 * n {
            result.push(path[..].iter().collect::<String>());
            return;
        }
        if left_used < n {
            path[k as usize] = '(';
            Self::backtrack(n, left_used + 1, right_used, k + 1, path, result);
        }
        if left_used > right_used {
            path[k as usize] = ')';
            Self::backtrack(n, left_used, right_used + 1, k + 1, path, result);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::generate_parenthesis(1), vec!["()"]);
        assert_eq!(Solution::generate_parenthesis(2), vec!["(())", "()()"]);
        assert_eq!(
            Solution::generate_parenthesis(3),
            vec!["((()))", "(()())", "(())()", "()(())", "()()()"]
        );
    }
}
