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
  // 遍历一遍 快指针走两步，慢指针走一步，慢指针停下来的地方就是中间结点。
  pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut head = head;
    let mut slow = head.as_ref();
    let mut fast = head.as_ref();

    loop {
      if let Some(node) = fast {
        fast = node.next.as_ref();
      } else {
        break;
      }
      if let Some(node) = fast {
        fast = node.next.as_ref();
      } else {
        break;
      }
      if let Some(node) = slow {
        slow = node.next.as_ref();
      } else {
        break;
      }
    }

    let mid_addr = if let Some(node) = slow {
      node.as_ref() as *const ListNode
    } else {
      return None;
    };

    while let Some(node) = head.as_ref() {
      let addr = node.as_ref() as *const ListNode;
      if addr != mid_addr {
        head = head.unwrap().next;
      } else {
        break;
      }
    }
    head
  }

  // 遍历两遍
  pub fn middle_node_one_pass(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut head = head;
    let mut cur = &head;
    let mut total = 0; // 获取总长度

    while let Some(inner) = cur {
      total += 1;
      cur = &inner.next;
    }
    if total % 2 == 0 {
      total = total / 2
    } else {
      total = (total - 1) / 2
    }

    let mut step = 0; // 根据总长度算出中间的位置
    while step < total {
      step += 1;
      if let Some(_head) = head {
        head = _head.next;
      }
    }
    head
  }
  // 遍历两遍
  pub fn middle_node_one_pass_2(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut head = head;
    let mut total = 0; // 获取总长度
    let mut cur = &mut head;
    loop {
      if cur.is_none() {
        break;
      }
      total += 1;
      cur = &mut cur.as_mut().unwrap().next;
    }
    cur = &mut head;
    for _ in 0..total / 2 {
      cur = &mut cur.as_mut().unwrap().next;
    }
    cur.take()
  }
}
#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn it_works() {
    assert_eq!(
      Solution::middle_node(linked![1, 2, 3, 4, 5, 6]),
      linked![4, 5, 6]
    );
    assert_eq!(
      Solution::middle_node_one_pass(linked![1, 2, 3, 4, 5, 6]),
      linked![4, 5, 6]
    );
    assert_eq!(
      Solution::middle_node_one_pass_2(linked![1, 2, 3, 4, 5, 6]),
      linked![4, 5, 6]
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
