struct Solution {}

impl Solution {
  pub fn calculate(s: String) -> i32 {
    let mut stack: Vec<i64> = Vec::new();
    let mut pre_sign = '+';
    let mut num = 0_i64;
    let mut ans = 0;

    for (i, ch) in s.chars().enumerate() {
      let is_digit = '0' <= ch && ch <= '9';
      if is_digit {
        num = num * 10 + (ch as u8 - '0' as u8) as i64
      }
      if !is_digit && ch != ' ' || i == s.len() - 1 {
        match pre_sign {
          '+' => stack.push(num),
          '-' => stack.push(-num),
          '*' => *stack.last_mut().unwrap() *= num,
          '/' => *stack.last_mut().unwrap() /= num,
          _ => (),
        }
        pre_sign = ch;
        num = 0;
      }
    }
    for x in stack {
      ans += x;
    }
    ans as i32
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    assert_eq!(7, Solution::calculate("3+2+2".to_owned()));
    assert_eq!(1, Solution::calculate("3/2".to_owned()));
    assert_eq!(5, Solution::calculate("3+5/2".to_owned()));
  }
}
