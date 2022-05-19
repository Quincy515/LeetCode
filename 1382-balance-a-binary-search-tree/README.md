# [1382.将二叉搜索树变平衡](https://leetcode.cn/problems/balance-a-binary-search-tree/description/)

给你一棵二叉搜索树，请你返回一棵 **平衡后** 的二叉搜索树，新生成的树应该与原来的树有着相同的节点值。如果有多种构造方法，请你返回任意一种。

如果一棵二叉搜索树中，每个节点的两棵子树高度差不超过 `1` ，我们就称这棵二叉搜索树是 **平衡的** 。

 

**示例 1：**

![img](https://assets.leetcode.com/uploads/2021/08/10/balance1-tree.jpg)

```
输入：root = [1,null,2,null,3,null,4,null,null]
输出：[2,1,3,null,null,null,4]
解释：这不是唯一的正确答案，[3,1,4,null,2,null,null] 也是一个可行的构造方案。
```

**示例 2：**

![img](https://assets.leetcode.com/uploads/2021/08/10/balanced2-tree.jpg)

```
输入: root = [2,1,3]
输出: [2,1,3]
```

 

**提示：**

- 树节点的数目在 `[1, 104]` 范围内。
- `1 <= Node.val <= 105`

------

[Discussion](https://leetcode.cn/problems/balance-a-binary-search-tree/comments/) | [Solution](https://leetcode.cn/problems/balance-a-binary-search-tree/solution/)

**思路**

首先将二叉树执行中序遍历以后得到一个有序数组，然后将[有序数组变成二叉搜索树](../0108-convert-sorted-array-to-binary-search-tree)

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
    pub fn balance_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut vec = vec![];
        inorder(root, &mut vec);
        sorted_array_to_bst(vec)
    }
}

fn inorder(root: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
    match root {
        Some(node) => {
            inorder(node.borrow().left.clone(), result);
            result.push(node.borrow().val);
            inorder(node.borrow().right.clone(), result);
        }
        None => {}
    }
}

fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    fn helper(nums: &[i32], l: usize, r: usize) -> Option<Rc<RefCell<TreeNode>>> {
        if l >= r {
            None
        } else {
            let mid = (l + r) / 2;
            let node = Rc::new(RefCell::new(TreeNode::new(nums[mid])));
            node.borrow_mut().left = helper(nums, l, mid);
            node.borrow_mut().right = helper(nums, mid + 1, r);
            Some(node)
        }
    }
    helper(&nums, 0, nums.len())
}
struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::balance_bst(tree![1, null, 2, null, 3, null, 4, null, null]),
            tree![2, 1, 3, null, null, null, 4]
        );
        assert_eq!(Solution::balance_bst(tree![2, 1, 3]), tree![2, 1, 3]);
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

