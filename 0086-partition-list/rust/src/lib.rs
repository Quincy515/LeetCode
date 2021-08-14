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
struct Solution;

impl Solution {
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut p = head;
        // 构建两个新链表
        // 大链表：大链表中的所有节点值都是大于或者等于特定值（除了虚拟头节点的值）
        // 小链表：小链表中的所有节点值都是小于特定值（除了虚拟头节点的值）

        // 设置一个指针，指向大链表的头结点
        let mut bigHead = Box::new(ListNode::new(-1));

        // 设置一个指针，指向大链表的尾结点
        let mut bigTail = &mut bigHead;

        // 设置一个指针，指向小链表的头结点
        let mut smallHead = Box::new(ListNode::new(-1));

        // 设置一个指针，指向小链表的尾结点
        let mut smallTail = &mut smallHead;

        // 开始遍历原链表 head，直到遍历到尾部位置
        // 在遍历的过程查看当前节点的值
        while let Some(mut cur) = p {
            p = cur.next.take();
            // 如果当前节点的值小于了特定值 x
            if cur.val < x {
                // 那么我们就把这个节点添加到小链表中
                // 操作就是让小链表中的尾节点的 next 指针指向这个节点
                smallTail.next = Some(cur);
                // 同时，小链表中的尾节点位置发生了变化，也移动到 head 这个位置
                smallTail = smallTail.next.as_mut().unwrap();
            } else {
                // 否则，如果当前节点的值大于或者等于了特定值 x
                // 那么我们就把这个节点添加到大链表中
                // 操作就是让大链表中的尾节点的 next 指针指向这个节点
                bigTail.next = Some(cur);
                // 同时，大链表中的尾节点位置发生了变化，也移动到 head 这个位置
                bigTail = bigTail.next.as_mut().unwrap();
            }

            // 操作完当前节点的值之后，继续去查看链表中的下一个节点
        }

        // 通过上面的循环，原链表已经被分割为两个部分
        // 其中，大链表中的所有节点值都是大于或者等于特定值（除了虚拟头节点的值）
        // 小链表中的所有节点值都是小于特定值（除了虚拟头节点的值）
        // 接下来，我们把大小链表串联起来

        // 让小链表的尾节点的 next 指针指向大链表虚拟头节点的下一个节点
        smallTail.next = bigHead.next;

        // 让大链表的尾节点的 next 指针指向 null

        // 最后返回小链表的虚拟头节点的下一个节点就行
        return smallHead.next;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(
            linked![1, 2, 2, 4, 3, 5],
            Solution::partition(linked![1, 4, 3, 2, 5, 2], 3)
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
