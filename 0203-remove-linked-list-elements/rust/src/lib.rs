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
  // 使用虚拟头节点
  pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
    let mut dummy = ListNode { val: 0, next: head };
    let mut p = &mut dummy; // dummy.as_mut();
    while let Some(inner) = p.next.as_mut() {
      if inner.val == val {
        p.next = inner.next.take();
      } else {
        p = p.next.as_mut().unwrap();
      }
    }
    dummy.next
  }
  // 不适用虚拟头节点
  pub fn remove_elements_2(mut head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
    while head.is_some() && head.as_ref().unwrap().val == val {
      head = head.unwrap().next;
    }
    if head.is_none() {
      return head;
    }
    let mut prev = &mut head;
    while prev.as_ref().unwrap().next.is_some() {
      if prev.as_ref().unwrap().next.as_ref().unwrap().val == val {
        let next = std::mem::replace(&mut prev.as_mut().unwrap().next, None);
        std::mem::replace(&mut prev.as_mut().unwrap().next, next.unwrap().next);
      } else {
        prev = &mut prev.as_mut().unwrap().next;
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
    assert_eq!(
      Solution::remove_elements(linked![1, 2, 6, 3, 4, 5, 6], 6),
      linked![1, 2, 3, 4, 5]
    );
    assert_eq!(
      Solution::remove_elements_2(linked![1, 2, 6, 3, 4, 5, 6], 6),
      linked![1, 2, 3, 4, 5]
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
