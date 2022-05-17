// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_even_odd_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let level_node = Self::level_order(root);
        for i in 0..level_node.len() {
            if (i & 1) != 0 {
                // 偶数，严格递减
                for j in 0..level_node[i].len() {
                    if (level_node[i][j] & 1) != 0 {
                        return false;
                    }
                    if j > 0 && level_node[i][j] >= level_node[i][j - 1] {
                        return false;
                    }
                }
            } else {
                // 奇数，单调递增
                for j in 0..level_node[i].len() {
                    if (level_node[i][j] & 1) == 0 {
                        return false;
                    }
                    if j > 0 && level_node[i][j] <= level_node[i][j - 1] {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut levels = vec![];
        if root.is_none() {
            return levels;
        }

        let mut deque = std::collections::VecDeque::new();
        deque.push_back(root);

        while !deque.is_empty() {
            // 开始当前层
            let mut current_level = vec![];

            // 当前层中的元素个数
            let level_length = deque.len();
            for _ in 0..level_length {
                let n = deque.pop_front();
                if let Some(Some(node)) = n {
                    // 添加当前节点
                    current_level.push(node.borrow().val);

                    // 将当前节点的左、右子节点加入队列
                    if node.borrow().left.is_some() {
                        deque.push_back(node.borrow().left.clone());
                    }
                    if node.borrow().right.is_some() {
                        deque.push_back(node.borrow().right.clone());
                    }
                }
            }
            levels.push(current_level);
        }
        levels
    }
}
struct Solution;
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert!(Solution::is_even_odd_tree(tree![
            1, 10, 4, 3, null, 7, 9, 12, 8, 6, null, null, 2
        ]));
        assert!(!Solution::is_even_odd_tree(tree![5, 4, 2, 3, 3, 7]));
        assert!(!Solution::is_even_odd_tree(tree![5, 9, 1, 3, 5, 7]));
        assert!(Solution::is_even_odd_tree(tree![1]));
        assert!(Solution::is_even_odd_tree(tree![
            11, 8, 6, 1, 3, 9, 11, 30, 20, 18, 16, 12, 10, 4, 2, 17
        ]));
    }
}
pub fn to_tree(vec: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    use std::collections::VecDeque;
    let head = Some(Rc::new(RefCell::new(TreeNode::new(vec[0].unwrap()))));
    let mut queue = VecDeque::new();
    queue.push_back(head.as_ref().unwrap().clone());

    for children in vec[1..].chunks(2) {
        let parent = queue.pop_front().unwrap();
        if let Some(v) = children[0] {
            parent.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(v))));
            queue.push_back(parent.borrow().left.as_ref().unwrap().clone());
        }
        if children.len() > 1 {
            if let Some(v) = children[1] {
                parent.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(v))));
                queue.push_back(parent.borrow().right.as_ref().unwrap().clone());
            }
        }
    }
    head
}

#[macro_export]
macro_rules! tree {
    () => {
        None
    };
    ($($e:expr),*) => {
        {
            let vec = vec![$(stringify!($e)), *];
            let vec = vec.into_iter().map(|v| v.parse::<i32>().ok()).collect::<Vec<_>>();
            to_tree(vec)
        }
    };
    ($($e:expr,)*) => {(tree![$($e),*])};
}
