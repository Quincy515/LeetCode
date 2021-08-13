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
    pub fn reverse_between(
        head: Option<Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode>> {
        unimplemented!();
        let mut head = head;
        // 一开始设置一个虚拟节点，它的值为 -1，它的值可以设置为任何的数，因为我们根本不需要使用它的值
        // 设置虚拟节点的目的是为了让原链表中所有节点就都可以按照统一的方式进行翻转
        // 比如如果翻转的区间包含了原链表中的第一个位置，那么如果不设置 dummy
        // 在翻转的过程中需要设置其它的临时变量来保持第一位置节点的指针
        // 具体可以通过动手画图来理解
        let mut dummy = Some(Box::new(ListNode::new(-1)));

        // 让虚拟节点指向原链表的头部
        dummy.as_mut().unwrap().next = head.clone();

        // 设置一个指针，指向以虚拟头节点为链表的头部位置
        let mut pre = &mut dummy;

        // 设置一个指针，指向原链表的头部位置
        let mut cur = &mut head;

        // 从虚拟头节点出发，pre 走 left - 1 步找到需要翻转的左区间
        // for 循环结束后，pre 的右节点是需要翻转的节点
        // for 循环结束后，cur 指向的就是需要翻转的节点
        for _ in 0..left-1 {
            // pre 不断的向右移动，直到走到翻转的左区间为止
            pre = &mut pre.as_mut().unwrap().next;
            // cur 不断的向右移动，找到了需要翻转的第一个节点
            cur = &mut cur.as_mut().unwrap().next;
        }

        // 开始翻转这些节点
        for _ in 0..right-left {
            // 设置临时变量，保存当前需要翻转节点的后面的节点
            let mut temp = cur.as_mut().unwrap().next.take();
            // 这个时候，让 temp 和 cur 两个节点翻转一下
            // 比如，一开始 i = 0 的时候 cur 为 2， temp 为 3
            // 执行完下面的代码，如果原链表是
            // 1 --> 2 --> 3 --> 4 --> 5
            // 变成了
            // 1 --> 3 --> 2 --> 4 --> 5

            // cur 的下一节点是等号右侧的值
            // i = 0 的时候， cur 为 2，cur.next.next 的值是 4
            // 所以，这行代码让 cur 的下一节点不是 3 ，而是 4
            // 2 --> 4
            // 等价于 cur.next = temp.next
            cur.as_mut().unwrap().next = cur.as_mut().unwrap().next.as_mut().unwrap().next.take();

            // temp 的下一节点是等号右侧的值
            // i = 0 的时候， temp 为 3，pre 为 1，pre 下一节点的值是 2
            // 所以，这行代码让 temp 的下一节点不是 4 ，而是 2
            // 3 --> 2
            // temp.next = pre.next;
            temp.as_mut().unwrap().next = pre.as_mut().unwrap().next.take();

            // pre 的下一节点是等号右侧的值
            // i = 0 的时候， pre 为 1，temp 的值是 3
            // 所以，这行代码让 pre 的下一节点为 3
            // 1 --> 3
            // pre.next = temp;
            pre.as_mut().unwrap().next = temp;

            // i = 0 结束之后，链表变成了
            // 1 --> 3 --> 2 --> 4 --> 5
        }

        // 最后返回虚拟头节点的下一个节点，因为虚拟节点不在链表中
        dummy.unwrap().next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::reverse_between(linked![1, 2, 3, 4, 5], 2, 4),
            linked![1, 4, 3, 2, 5]
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
