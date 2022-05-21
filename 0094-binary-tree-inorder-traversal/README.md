# [94.二叉树的中序遍历](https://leetcode.cn/problems/binary-tree-inorder-traversal/description/)

给定一个二叉树的根节点 `root` ，返回 *它的 **中序** 遍历* 。

 

**示例 1：**

![img](https://assets.leetcode.com/uploads/2020/09/15/inorder_1.jpg)

```
输入：root = [1,null,2,3]
输出：[1,3,2]
```

**示例 2：**

```
输入：root = []
输出：[]
```

**示例 3：**

```
输入：root = [1]
输出：[1]
```

 

**提示：**

- 树中节点数目在范围 `[0, 100]` 内
- `-100 <= Node.val <= 100`

 

**进阶:** 递归算法很简单，你可以通过迭代算法完成吗？

------

[Discussion](https://leetcode.cn/problems/binary-tree-inorder-traversal/comments/) | [Solution](https://leetcode.cn/problems/binary-tree-inorder-traversal/solution/)

**题解**

**递归思路**

先递归访问左子树，然后访问根节点，最后递归访问右子树，即实现左子树->根节点->右子树的访问

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
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = vec![];
        if root.is_none() {
            return result;
        }

        inorder_recursive(root, &mut result);
        result
    }
}

fn inorder_recursive(root: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
    match root {
        Some(node) => {
            // 递归遍历左子树
            inorder_recursive(node.borrow().left.clone(), result);
            // 访问当前节点
            result.push(node.borrow().val);
            // 递归遍历右子树
            inorder_recursive(node.borrow().right.clone(), result);
        }
        None => {}
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::inorder_traversal(tree![1, null, 2, 3]),
            vec![1, 3, 2]
        );
        assert_eq!(Solution::inorder_traversal(tree![]), vec![]);
        assert_eq!(Solution::inorder_traversal(tree![1]), vec![1]);
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



**非递归思路**

利用栈的数据结构特性来保存需要返回后处理的节点，优先遍历左子树节点。在遍历过程中将当前节点入栈，直到左子树为空。再讲栈顶的节点出栈，并进入其右子树继续进行遍历。

非递归实现的算法流程如下：

1、创建一个栈用来存放节点

2、若当前节点非空，将当前节点入栈，并进入其左子树访问

3、重复步骤2，直到当前节点为空

4、将栈顶的节点出栈，访问其节点值，并进入其右子树访问

5、重复步骤2~4，直到当前节点为空且栈为空，完成所有节点的访问

```rust
impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = vec![];
        if root.is_none() {
            return result;
        }

        // 使用栈来保存需要返回后处理的节点
        let mut stack = Vec::new();
        let mut r = root.clone();

        // 满足当前节点非空或者栈非空时执行循环
        while r.is_some() || !stack.is_empty() {
            // 若当前节点非空，将当前节点入栈，并进入其左子树访问
            while let Some(node) = r {
                stack.push(node.clone());
                r = node.borrow().left.clone();
            }

            // 栈顶的节点出栈，访问其节点值，并进入其右子树访问
            r = stack.pop();
            if let Some(node) = r {
                result.push(node.borrow().val);
                r = node.borrow().right.clone();
            }
        }

        result
    }
}
```

