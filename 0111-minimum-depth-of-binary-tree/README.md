# [111.二叉树的最小深度](https://leetcode.cn/problems/minimum-depth-of-binary-tree/description/)

给定一个二叉树，找出其最小深度。

最小深度是从根节点到最近叶子节点的最短路径上的节点数量。

**说明：**叶子节点是指没有子节点的节点。

 

**示例 1：**

![img](https://assets.leetcode.com/uploads/2020/10/12/ex_depth.jpg)

```
输入：root = [3,9,20,null,null,15,7]
输出：2
```

**示例 2：**

```
输入：root = [2,null,3,null,4,null,5,null,6]
输出：5
```

 

**提示：**

- 树中节点数的范围在 `[0, 105]` 内
- `-1000 <= Node.val <= 1000`

------

[Discussion](https://leetcode.cn/problems/minimum-depth-of-binary-tree/comments/) | [Solution](https://leetcode.cn/problems/minimum-depth-of-binary-tree/solution/)

**思路**

根节点到叶子节点的最小深度可使用深度优先搜索算法以递归的方式计算。其分为以下3种情况。

1、当左、右子树都为空时，只有1个根节点，深度为1（根节点与叶子节点重合）

2、当左、右子树有一个为空时，返回的是非空子树的最小深度，而不是空子树的深度0。若返回0相当于把当前节点视成叶子节点，与此节点有非空子树矛盾

3、当左、右子树都不为空时，返回左、右子树深度的较小值

**题解**

```rust
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
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(node) => {
                // 左子树为空，返回右子树的最小深度
                if node.borrow().left.is_none() {
                    return Self::min_depth(node.borrow().right.clone()) + 1;
                }
                // 右子树为空，返回左子树的最小深度
                if node.borrow().right.is_none() {
                    return Self::min_depth(node.borrow().left.clone()) + 1;
                }

                // 左、右子树都不为空，返回左、右子树深度的较小值
                let left = Self::min_depth(node.borrow().left.clone());
                let right = Self::min_depth(node.borrow().right.clone());
                left.min(right) + 1
            }
            None => 0,
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::min_depth(tree![3, 9, 20, null, null, 15, 7]), 2);
        assert_eq!(
            Solution::min_depth(tree![2, null, 3, null, 4, null, 5, null, 6]),
            5
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
```

