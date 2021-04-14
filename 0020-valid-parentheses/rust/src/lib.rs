struct Solution;

impl Solution {
  pub fn is_valid(s: String) -> bool {
    let s: Vec<char> = s.chars().collect();
    let mut stack = vec![];
    for i in 0..s.len() {
      let c = s[i];
      if c == '(' || c == '[' || c == '{' {
        stack.push(c)
      } else {
        if stack.len() == 0 {
          return false;
        }
        if stack.len() > 0 {
          let top_char = stack.pop().unwrap();
          if c == ')' && top_char != '(' {
            return false;
          }
          if c == ']' && top_char != '[' {
            return false;
          }
          if c == '}' && top_char != '{' {
            return false;
          }
        }
      }
    }
    stack.len() == 0
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn it_works() {
    assert_eq!(true, Solution::is_valid("()".to_owned()));
    assert_eq!(true, Solution::is_valid("()[]{}".to_owned()));
    assert_eq!(false, Solution::is_valid("(]".to_owned()));
    assert_eq!(false, Solution::is_valid("([)]".to_owned()));
    assert_eq!(true, Solution::is_valid("{[]}".to_owned()));
  }
}
