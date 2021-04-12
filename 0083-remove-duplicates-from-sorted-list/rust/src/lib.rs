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
  pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none() {
      return head;
    }
    let mut head = head;
    let mut cur = &mut head; // 慢指针初始化指向第一个节点
    while cur.is_some() && cur.as_ref().unwrap().next.is_some() {
      if cur.as_ref().unwrap().val == cur.as_ref().unwrap().next.as_ref().unwrap().val {
        *cur = cur.as_mut().unwrap().next.take();
      } else {
        cur = &mut cur.as_mut().unwrap().next;
      }
    }
    head
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn it_works() {
    assert_eq!(linked![1, 2], Solution::delete_duplicates(linked![1, 1, 2]));
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
