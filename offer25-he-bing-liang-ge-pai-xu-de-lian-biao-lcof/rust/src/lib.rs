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
  pub fn merge_two_lists(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
  ) -> Option<Box<ListNode>> {
    let (mut l1, mut l2) = (l1, l2);
    let mut dummy = Some(Box::new(ListNode::new(0)));
    let mut p = &mut dummy;

    while l1.is_some() && l2.is_some() {
      if l1.as_ref().unwrap().val < l2.as_ref().unwrap().val {
        p.as_mut().unwrap().next = l1.take();
        l1 = p.as_mut().unwrap().next.as_mut().unwrap().next.take();
      } else {
        p.as_mut().unwrap().next = l2.take();
        l2 = p.as_mut().unwrap().next.as_mut().unwrap().next.take();
      }
      p = &mut p.as_mut().unwrap().next;
    }
    if !l1.is_none() {
      p.as_mut().unwrap().next = l1;
    }
    if !l2.is_none() {
      p.as_mut().unwrap().next = l2;
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
      linked![1, 1, 2, 3, 4, 4],
      Solution::merge_two_lists(linked![1, 2, 4], linked![1, 3, 4])
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
