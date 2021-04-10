struct Solution {}

impl Solution {
  pub fn can_jump(num: Vec<i32>) -> bool {
    if num.is_empty() {
      return false;
    }
    let (n, mut max) = (num.len(), 0); // 定义 max 最远可以跳到的下标
    for i in 0..n {
      if max >= n - 1 { // 最远可达下标已经大于等于 n-1
        return true; // 提前返回 true
      }
      if i > max { // 若当前下标 i 已经超出了最远可达下标
        return false; // 提前返回 false
      }// 若不是上面两种情况，就更新 max 最远可达下标
      max = std::cmp::max(max, i + num[i] as usize);
    }
    false // 循环结束后还没有提前返回就返回 false
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    assert_eq!(true, Solution::can_jump(vec![2, 3, 1, 1, 4]));
    assert_eq!(false, Solution::can_jump(vec![3, 2, 1, 0, 4]));
  }
}
