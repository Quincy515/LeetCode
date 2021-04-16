struct Solution {}

impl Solution {
  pub fn daily_temperatures(t: Vec<i32>) -> Vec<i32> {
    let mut ans = vec![0i32; t.len()];
    let mut stack = vec![];
    for i in 0..t.len() {
      let temperature = t[i];
      while stack.len() > 0 && temperature > t[stack[stack.len() - 1]] {
        let prev_index = stack.pop().unwrap();
        ans[prev_index] = (i - prev_index) as i32;
      }
      stack.push(i);
    }
    ans
  }
  pub fn daily_temperatures_tuple(t: Vec<i32>) -> Vec<i32> {
    let mut stack = vec![];
    let mut ret = vec![0; t.len()];

    for i in (0..t.len()).rev() {
      while stack.last().unwrap_or(&(101, 0)).0 <= t[i] {
        stack.pop();
      }
      ret[i] = (stack.last().unwrap_or(&(0, i)).1 - i) as i32;
      stack.push((t[i], i));
    }

    ret
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn it_works() {
    assert_eq!(
      Solution::daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]),
      vec![1, 1, 4, 2, 1, 1, 0, 0]
    );
    assert_eq!(
      Solution::daily_temperatures_tuple(vec![73, 74, 75, 71, 69, 72, 76, 73]),
      vec![1, 1, 4, 2, 1, 1, 0, 0]
    );
  }
}
