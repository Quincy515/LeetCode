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
  pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
  ) -> Option<Box<ListNode>> {
    let (mut l1, mut l2) = (l1, l2);
    let mut dummy = Some(Box::new(ListNode::new(0)));
    let mut carry = 0;
    let mut p = &mut dummy;
    while l1.is_some() || l2.is_some() || carry != 0 {
      let mut sum = carry;
      if let Some(v) = l1 {
        sum += v.val;
        l1 = v.next;
      }
      if let Some(v) = l2 {
        sum += v.val;
        l2 = v.next
      }
      if let Some(inner) = p {
        inner.next = Some(Box::new(ListNode::new(sum % 10)));
        p = &mut inner.next;
      }
      carry = sum / 10
    }
    dummy.unwrap().next
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn it_works() {
    assert_eq!(
      linked![7, 0, 8],
      Solution::add_two_numbers(to_list(vec![2, 4, 3]), to_list(vec![5, 6, 4]))
    );
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
