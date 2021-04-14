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
  pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut head = head;
    let cur = &mut head;
    let mut prev = None;
    while cur.is_some() {
      let mut next = std::mem::replace(&mut cur.as_mut().unwrap().next, prev.take());
      prev = cur.take();
      std::mem::swap(&mut next, cur);
    }
    prev
  }

  pub fn reverse_list_2(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut cur = head;
    let mut prev = None;
    while let Some(mut node) = cur {
      cur = node.next.take();
      node.next = prev;
      prev = Some(node);
    }
    prev
  }
}
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    assert_eq!(
      linked![5, 4, 3, 2, 1],
      Solution::reverse_list(linked![1, 2, 3, 4, 5])
    );
    assert_eq!(
      linked![5, 4, 3, 2, 1],
      Solution::reverse_list_2(linked![1, 2, 3, 4, 5])
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
