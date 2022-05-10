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

struct Solution;

impl Solution {
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let money = Self::postorder(root);
        money[0].max(money[1])
    }
    fn postorder(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut money = vec![0; 2];

        if root.is_none() {
            return money;
        }
        let left_money = Self::postorder(root.as_ref().unwrap().borrow().left.clone());
        let right_money = Self::postorder(root.as_ref().unwrap().borrow().right.clone());
        money[0] =
            i32::max(left_money[0], left_money[1]) + i32::max(right_money[0], right_money[1]);
        money[1] = (left_money[0] + right_money[0]) + root.as_ref().unwrap().borrow().val;
        money
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
