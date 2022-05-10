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
use std::collections::HashMap;
use std::rc::Rc;

struct Solution;

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        let mut count = 0;
        Self::dfs1(root, target_sum, &mut count);
        count
    }
    // 返回以 root 为上端点的路径的所有的长度 (key) 对应的路径的个数 (value)
    fn dfs1(root: Option<Rc<RefCell<TreeNode>>>, sum: i32, count: &mut i32) -> HashMap<i32, i32> {
        if root.is_none() {
            return HashMap::new();
        }
        let mut left_values = Self::dfs1(root.as_ref().unwrap().borrow().left.clone(), sum, count);
        let mut right_values =
            Self::dfs1(root.as_ref().unwrap().borrow().right.clone(), sum, count);
        let mut root_values = HashMap::new();
        root_values.insert(root.as_ref().unwrap().borrow().val, 1);

        for (key, val) in left_values.iter_mut() {
            let new_key = root.as_ref().unwrap().borrow().val + key;
            let mut new_value = *val;
            if root_values.contains_key(&new_key) {
                new_value += root_values.get(&new_key).unwrap();
            }
            root_values.insert(new_key, new_value);
        }
        for (key, val) in right_values.iter_mut() {
            let new_key = root.as_ref().unwrap().borrow().val + key;
            let mut new_value = *val;
            if root_values.contains_key(&new_key) {
                new_value += root_values.get(&new_key).unwrap();
            }
            root_values.insert(new_key, new_value);
        }
        for (key, value) in &root_values {
            if *key == sum {
                *count += *value;
            }
        }
        root_values
    }
    pub fn path_sum2(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        let mut count = 0;
        Self::dfs(root, &mut vec![], target_sum, &mut count);
        count
    }
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, path: &mut Vec<i32>, sum: i32, count: &mut i32) {
        if root.is_none() {
            return;
        }
        path.push(root.as_ref().unwrap().borrow().val);

        let mut cur_sum = 0;
        for i in (0..=path.len() - 1).rev() {
            cur_sum += path[i];
            if cur_sum == sum {
                *count += 1;
            }
        }

        Self::dfs(
            root.as_ref().unwrap().borrow().left.clone(),
            path,
            sum,
            count,
        );
        Self::dfs(
            root.as_ref().unwrap().borrow().right.clone(),
            path,
            sum,
            count,
        );

        path.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
