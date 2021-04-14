// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>,
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode { next: None, val }
  }
}

struct Solution {}
impl Solution {
  pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
    let mut stack = Vec::new();
    let mut p = head;
    while let Some(node) = p {
      stack.push(node.val);
      p = node.next;
    }
    let (mut i, mut j) = (0, stack.len());
    while i < j {
      if stack[i] != stack[j - 1] {
        return false;
      }
      i += 1;
      j -= 1;
    }
    true
  }
}
#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn it_works() {
    assert_eq!(false, Solution::is_palindrome(linked![1, 2]));
  }
}

// helper function for test
pub fn to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
  let mut current = None;
  for &v in vec.iter().rev() {
    let mut node = ListNode::new(v);
    node.next = current;
    current = Some(Box::new(node));
  }
  current
}

#[macro_export]
macro_rules! linked {
  ($($e:expr),*) => {to_list(vec![$($e.to_owned()), *])};
  ($($e:expr,)*) => {to_list(vec![$($e.to_owned()), *])};
}
