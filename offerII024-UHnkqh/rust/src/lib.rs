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
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None;
        let mut curr = head;

        while let Some(mut curr_node) = curr.take() {
            // 保存当前节点的下一个节点
            let next_temp = curr_node.next.take();
            // 将当前节点指向 prev 节点
            curr_node.next = prev.take();

            // prev 和 curr 分别往后移动一个节点，即
            // 把当前节点 curr_node 赋值给 prev
            // 把之前保存的当前节点的下一个节点 next_temp 赋值给 curr
            prev = Some(curr_node);
            curr = next_temp;
        }

        prev
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
