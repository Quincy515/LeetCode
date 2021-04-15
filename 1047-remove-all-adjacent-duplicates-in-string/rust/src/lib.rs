use std::iter::FromIterator;
struct Solution;
impl Solution {
  pub fn remove_duplicates(s: String) -> String {
    let mut stack: Vec<char> = Vec::new();
    for ch in s.chars() {
      if stack.len() == 0 || stack.last_mut().unwrap() != &ch {
        stack.push(ch);
      } else {
        stack.pop();
      }
    }
    String::from_iter(stack)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn it_works() {
    assert_eq!(
      "ca".to_owned(),
      Solution::remove_duplicates("abbaca".to_owned())
    );
  }
}
