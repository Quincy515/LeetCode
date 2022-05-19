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
    pub fn insert_into_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        // 如果根节点为空，直接返回由插入值创建的节点
        if root.is_none() {
            return Some(Rc::new(RefCell::new(TreeNode::new(val))));
        }

        insert(&root, val);
        root
    }
}

fn insert(root: &Option<Rc<RefCell<TreeNode>>>, val: i32) {
    if let Some(node) = root {
        let mut n = node.borrow_mut();

        // val 大于当前节点值，往右子树查找
        // val 小于当前节点值，往左子树查找
        let target = if val > n.val {
            &mut n.right
        } else {
            &mut n.left
        };
        if target.is_some() {
            return insert(target, val);
        }

        // 在找到的空节点位置插入
        *target = Some(Rc::new(RefCell::new(TreeNode::new(val))));
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::insert_into_bst(tree![4, 2, 7, 1, 3], 5),
            tree![4, 2, 7, 1, 3, 5]
        );
        assert_eq!(
            Solution::insert_into_bst(tree![40, 20, 60, 10, 30, 50, 70], 25),
            tree![40, 20, 60, 10, 30, 50, 70, null, null, 25]
        );
        assert_eq!(
            Solution::insert_into_bst(tree![4, 2, 7, 1, 3, null, null, null, null, null, null], 5),
            tree![4, 2, 7, 1, 3, 5]
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
