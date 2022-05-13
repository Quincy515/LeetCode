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
    pub fn next_larger_nodes(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut stk: Vec<(i32, i32)> = Vec::new();
        let mut result = Vec::new();

        let mut curr = head;
        let mut i = 0;
        while let Some(mut curr_node) = curr {
            result.push(0);
            while !stk.is_empty() && curr_node.val > stk.last().unwrap().0 {
                let index = stk.last().unwrap().1;
                result[index as usize] = curr_node.val;
                stk.pop();
            }
            stk.push((curr_node.val, i as i32));
            curr = curr_node.next.take();
            i += 1;
        }
        result
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
