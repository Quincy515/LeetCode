// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}
struct Solution;

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // 构建一个链表用来存放 l1 和 l2 两个链表相加的结果
        // 其中 dummy 这个节点为虚拟头结点
        let mut dummy = ListNode::new(0);

        // 设置一个进位，初始化为 0
        // 两个个位数相加，进位只能是 1 或者 0
        // 比如 7 + 8 = 15，进位是 1
        // 比如 2 + 3 = 6，没有进位，或者说进位是 0
        let mut carry_bit = 0;

        let (mut p, mut q) = (l1, l2);
        let (mut v1, mut v2) = (Vec::new(), Vec::new());

        while p.is_some() || q.is_some() {
            // 获取 l1 链表中节点的值
            if let Some(v) = p {
                v1.push(v.val);
                p = v.next;
            }

            // 获取 l2 链表中节点的值
            if let Some(v) = q {
                v2.push(v.val);
                q = v.next;
            }
        }

        while v1.len() > 0 || v2.len() > 0 || carry_bit > 0 {
            let mut sum = carry_bit;
            if !v1.is_empty() {
                sum += v1.pop().unwrap();
            }
            if !v2.is_empty() {
                sum += v2.pop().unwrap();
            }

            let mut node = ListNode::new(sum % 10);
            node.next = dummy.next.take();
            dummy.next = Some(Box::new(node));
            carry_bit = sum / 10;
        }
        dummy.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::add_two_numbers(linked![2, 4, 3], linked![5, 6, 4]),
            linked![8, 0, 7]
        );
        assert_eq!(
            Solution::add_two_numbers(linked![5], linked![5]),
            linked![1, 0]
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

