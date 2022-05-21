# [701.二叉搜索树中的插入操作](https://leetcode.cn/problems/insert-into-a-binary-search-tree/description/)

给定二叉搜索树（BST）的根节点 `root` 和要插入树中的值 `value` ，将值插入二叉搜索树。 返回插入后二叉搜索树的根节点。 输入数据 **保证** ，新值和原始二叉搜索树中的任意节点值都不同。

**注意**，可能存在多种有效的插入方式，只要树在插入后仍保持为二叉搜索树即可。 你可以返回 **任意有效的结果** 。

 

**示例 1：**

![img](https://assets.leetcode.com/uploads/2020/10/05/insertbst.jpg)

```
输入：root = [4,2,7,1,3], val = 5
输出：[4,2,7,1,3,5]
解释：另一个满足题目要求可以通过的树是：
```

**示例 2：**

```
输入：root = [40,20,60,10,30,50,70], val = 25
输出：[40,20,60,10,30,50,70,null,null,25]
```

**示例 3：**

```
输入：root = [4,2,7,1,3,null,null,null,null,null,null], val = 5
输出：[4,2,7,1,3,5]
```

 

**提示：**

- 树中的节点数将在 `[0, 104]`的范围内。
- `-108 <= Node.val <= 108`
- 所有值 `Node.val` 是 **独一无二** 的。
- `-108 <= val <= 108`
- **保证** `val` 在原始BST中不存在。

------

[Discussion](https://leetcode.cn/problems/insert-into-a-binary-search-tree/comments/) | [Solution](https://leetcode.cn/problems/insert-into-a-binary-search-tree/solution/)

**思路**

如果二叉树为空树，直接用 val 构造二叉树节点并将其作为根节点返回。

如果要给新值找到合适的插入位置，可以将新节点作为当前二叉搜索树的某个叶子节点的子节点进行插入。插入到哪个叶子节点遵循以下原则。

- 若 `val > node.val`，值插入右子树
- 若 `val < node.val`，值插入左子树

在遇到应该走向左子树而左子树为空，或者应该走向右子树而右子树为空时，改节点就是新节点的插入位置了。

**流程**

1、判断根节点是否为空，如果为空，直接返回 `TreeNode(val)`

2、继续查找节点，分为两种情况

- 如果 `val > node.val`，进入该节点的右子树查找
- 如果 `val < node.val`，进入该节点的左子树查找

3、重复步骤2，直到节点为 None，插入 `TreeNode(val)`，返回根节点

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
```

