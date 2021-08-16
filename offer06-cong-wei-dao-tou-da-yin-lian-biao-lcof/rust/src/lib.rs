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
    pub fn reverse_print(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut p = head;
        // 构建一个栈，用来存储链表中每个节点的值
        let mut stack = Vec::new();

        // 构建一个指针，指向链表的头结点位置，从它开始向后遍历
        // 不断的遍历原链表中的每个节点，直到为 null
        while let Some(cur) = p {
            // 把每个节点的值加入到栈中
            stack.push(cur.val);
            // curNode 向后移动
            p = cur.next;
        }
        // stack.reverse();
        // stack
        // stack.iter().rev().map(|&x|x).collect::<Vec<i32>>()
        // 获取栈的长度
        let size = stack.len();
        // 通过栈的长度，定义一个同样长度的数组 res
        let mut res = Vec::with_capacity(size);
        // 遍历栈，从栈顶挨个弹出每个值，把这些值依次加入到数组 res 中
        while !stack.is_empty() {
            // 数组接收栈顶元素值
            res.push(stack.pop().unwrap());
        }
        // 最后返回结果数组就行
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::reverse_print(linked![1, 3, 2]),
            vec![2, 3, 1]
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
  