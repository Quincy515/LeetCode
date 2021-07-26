struct Solution;

impl Solution {
  pub fn is_valid(s: String) -> bool {
    let chars: Vec<char> = s.chars().collect();
    // 判断是否为空字符串，空字符串视为有效字符串
    if chars.len() == 0 { return true; }
    let mut stack: Vec<char> = Vec::new();
    for i in 0..chars.len() {
      // 如果是左小括号，将右小括号入栈
      if chars[i] == '(' { stack.push(')'); }
      // 如果是左中括号，将右中括号入栈
      else if chars[i] == '[' { stack.push(']'); }
      // 如果是左大括号，将右大括号入栈
      else if chars[i] == '{' { stack.push('}'); }
      // 栈为空或与栈顶元素不相同为无效字符串
      else if stack.is_empty() || chars[i] != stack.pop().unwrap() { return false; }
    }
    // 匹配结束，栈为空代表是有效字符串，否则是无效字符串
    return stack.is_empty();
  }
}

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
