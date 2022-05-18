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
    pub fn del_nodes(
        root: Option<Rc<RefCell<TreeNode>>>,
        to_delete: Vec<i32>,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut result = vec![];
        Solution::dfs(&root, true, &mut result, &to_delete);
        result
    }
    fn dfs(
        root: &Option<Rc<RefCell<TreeNode>>>,
        is_root: bool,
        result: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
        to_delete: &[i32],
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            None => None,
            Some(x) => {
                let mut node = x.borrow_mut();
                let val = node.val;

                let mut flag = false;
                for &i in to_delete {
                    if i == val {
                        flag = true;
                        break;
                    }
                }
                if !flag && is_root {
                    result.push(Some(x.clone()));
                }
                node.left = Self::dfs(&node.left, flag, result, to_delete);
                node.right = Self::dfs(&node.right, flag, result, to_delete);
                if flag {
                    None
                } else {
                    root.clone()
                }
            }
        }
    }
}
struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(
            Solution::del_nodes(tree![1, 2, 3, 4, 5, 6, 7], vec![3, 5]),
            vec![tree![1, 2, null, 4], tree![6], tree![7]]
        );
        assert_eq!(
            Solution::del_nodes(tree![1, 2, 4, null, 3], vec![3]),
            vec![tree![1, 2, 4]]
        );
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
