# [700.二叉搜索树中的搜索](https://leetcode.cn/problems/search-in-a-binary-search-tree/description/)

给定二叉搜索树（BST）的根节点 `root` 和一个整数值 `val`。

你需要在 BST 中找到节点值等于 `val` 的节点。 返回以该节点为根的子树。 如果节点不存在，则返回 `null` 。

 

**示例 1:**

![img](https://assets.leetcode.com/uploads/2021/01/12/tree1.jpg)

```
输入：root = [4,2,7,1,3], val = 2
输出：[2,1,3]
```

**Example 2:**

![img](https://assets.leetcode.com/uploads/2021/01/12/tree2.jpg)

```
输入：root = [4,2,7,1,3], val = 5
输出：[]
```

 

**提示：**

- 数中节点数在 `[1, 5000]` 范围内
- `1 <= Node.val <= 107`
- `root` 是二叉搜索树
- `1 <= val <= 107`

------

[Discussion](https://leetcode.cn/problems/search-in-a-binary-search-tree/comments/) | [Solution](https://leetcode.cn/problems/search-in-a-binary-search-tree/solution/)

**思路**

根据二次搜索树的特性，每个节点必须大于左子树上任意节点，小于右子树上任意节点，因此可以遍历此二次搜索树的每个节点。当目标值等于当前节点的值时，返回当前节点；当目标是小于当前节点的值时，进入其左子树继续查找；当目标值大于当前节点的值时，进入其右子树继续查找。如果到最后节点仍未找到目标值，则返回 None。

**流程**

1、判断根节点是否为空，如果为空，直接返回 None；如果不为空，判断根节点的值是否为目标值，如果是目标值则直接返回根节点

2、继续查找节点，分为3种情况：

- 如果 `val == node.val`，直接返回该节点
- 如果 `val > node.val`，则进入该节点的右子树查找
- 如果 `val < node.val`，则进入该节点的左子树查找

3、重复步骤2，直到最后仍未找到目标值，返回 None

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
      right: None
    }
  }
}
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn search_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        let mut r = root.clone();
        while let Some(node) = r {
            // 如果等于目标值，返回该节点
            if node.borrow().val == val { return Some(node); }

            // 如果大于目标值，搜索左子树，否则，搜索右子树
            if node.borrow().val > val  {
                r = node.borrow().left.clone();
            } else {
                r = node.borrow().right.clone();
            }
        }
        // 没找到，返回 None
        None
    }
}


struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::search_bst(tree![4,2,7,1,3],2), tree![2,1,3]);
        assert_eq!(Solution::search_bst(tree![4,2,7,1,3],5), tree![]);
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

