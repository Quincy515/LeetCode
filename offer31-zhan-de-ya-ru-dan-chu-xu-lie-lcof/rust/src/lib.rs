struct Solution;
impl Solution {
  pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
    if pushed.len() != popped.len() {
      return false;
    }
    let mut stack: Vec<i32> = Vec::new();
    let mut p = 0;
    for value in pushed {
      stack.push(value);
      while stack.len() > 0 && *stack.last_mut().unwrap() == popped[p] {
        stack.pop();
        p += 1;
      }
    }
    stack.len() == 0
  }

  // 用数组模拟栈
  pub fn validate_stack_sequences_array(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
    if pushed.len() != popped.len() {
      return false;
    }
    let mut stack = vec![0; pushed.len()];
    let mut top = -1_i32;
    let mut p = 0;
    for val in pushed {
      top += 1;
      stack[top as usize] = val;
      while top != -1 && stack[top as usize] == popped[p] {
        top -= 1;
        p += 1;
      }
    }
    top == -1
  }
}
#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn it_works() {
    assert_eq!(
      true,
      Solution::validate_stack_sequences(vec![1, 2, 3, 4, 5], vec![4, 5, 3, 2, 1])
    );
    assert_eq!(
      false,
      Solution::validate_stack_sequences(vec![1, 2, 3, 4, 5], vec![4, 5, 3, 1, 2])
    );
    assert_eq!(
      true,
      Solution::validate_stack_sequences_array(vec![1, 2, 3, 4, 5], vec![4, 5, 3, 2, 1])
    );
    assert_eq!(
      false,
      Solution::validate_stack_sequences_array(vec![1, 2, 3, 4, 5], vec![4, 5, 3, 1, 2])
    );
  }
}
