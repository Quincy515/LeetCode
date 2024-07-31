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
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (Some(node1), None) => Some(node1),
            (None, Some(node2)) => Some(node2),
            (Some(mut node1), Some(mut node2)) => {
                if node1.val < node2.val {
                    let n = node1.next.take();
                    node1.next = Solution::merge_two_lists(n, Some(node2));
                    Some(node1)
                } else {
                    let n = node2.next.take();
                    node2.next = Solution::merge_two_lists(Some(node1), n);
                    Some(node2)
                }
            }
            _ => None,
        }
    }

    pub fn merge_two_lists_v2(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let (mut l1, mut l2) = (l1, l2);
        // 一开始设置一个虚拟节点，它的值为 -1，它的值可以设置为任何的数，因为我们根本不需要使用它的值
        let mut dummy = Some(Box::new(ListNode::new(0)));
        // 设置一个指针，指向虚拟节点
        let mut p = &mut dummy;

        // 通过一个循环，不断的比较 l1 和 l2 中当前节点值的大小，直到 l1 或者 l2 遍历完毕为止
        while l1.is_some() && l2.is_some() {
            // 如果 l1 当前节点的值小于等于 l2 当前节点的值
            if l1.as_ref().unwrap().val < l2.as_ref().unwrap().val {
                // 让 pre 指向节点的 next 指针指向这个更小值的节点，即指向 l1
                p.as_mut().unwrap().next = l1.take();
                // 让 l1 向后移动
                l1 = p.as_mut().unwrap().next.as_mut().unwrap().next.take();
            } else {
                // 让 pre 指向节点的 next 指针指向这个更小值的节点，即指向 l2
                p.as_mut().unwrap().next = l2.take();
                // 让 l2 向后移动
                l2 = p.as_mut().unwrap().next.as_mut().unwrap().next.take();
            }
            // 让 pre 向后移动
            p = &mut p.as_mut().unwrap().next;
        }

        // 跳出循环后，l1 或者 l2 中可能有剩余的节点没有被观察过
        // 直接把剩下的节点加入到 pre 的 next 指针位置

        // 如果 l1 中还有节点
        if !l1.is_none() {
            // 把 l1 中剩下的节点全部加入到 pre 的 next 指针位置
            p.as_mut().unwrap().next = l1;
        }

        // 如果 l2 中海油节点
        if !l2.is_none() {
            // 把 l2 中剩下的节点全部加入到 pre 的 next 指针位置
            p.as_mut().unwrap().next = l2;
        }

        // 最后返回虚拟节点的 next 指针
        dummy.unwrap().next
    }

    pub fn merge_two_lists_v3(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut current = &mut dummy;
        while list1.is_some() && list2.is_some() {
            let l1 = list1.as_mut().unwrap();
            let l2 = list2.as_mut().unwrap();

            if l1.val <= l2.val {
                let next = l1.next.take();
                current.next = list1.take();
                list1 = next;
            } else {
                let next = l2.next.take();
                current.next = list2.take();
                list2 = next;
            }
            current = current.next.as_mut().unwrap();
        }
        if list1.is_some() {
            current.next = list1;
        } else if list2.is_some() {
            current.next = list2;
        }
        dummy.next
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(
            linked![1, 1, 2, 3, 4, 4],
            Solution::merge_two_lists(linked![1, 2, 4], linked![1, 3, 4])
        );
        assert_eq!(
            linked![1, 1, 2, 3, 4, 4],
            Solution::merge_two_lists_v3(linked![1, 2, 4], linked![1, 3, 4])
        );
        assert_eq!(
            linked![1, 1, 2, 3, 4, 4],
            Solution::merge_two_lists_v3(linked![1, 2, 4], linked![1, 3, 4])
        );
    }
}
