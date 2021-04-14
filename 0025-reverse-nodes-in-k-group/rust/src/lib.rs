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
  pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let mut head = head;
    let mut node = &mut head;
    for _ in 0..k {
      match node.as_mut() {
        None => return head,
        Some(_node) => node = &mut _node.next,
      }
    }
    let node = node.take();
    Solution::reverse(head, Solution::reverse_k_group(node, k))
  }

  fn reverse(head: Option<Box<ListNode>>, tail: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut cur = head;
    let mut prev = tail;
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
  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
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
