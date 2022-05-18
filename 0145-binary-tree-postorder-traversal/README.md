# [145.二叉树的后序遍历](https://leetcode.cn/problems/binary-tree-postorder-traversal/description/)

给你一棵二叉树的根节点 `root` ，返回其节点值的 **后序遍历** 。

 

**示例 1：**

![img](https://assets.leetcode.com/uploads/2020/08/28/pre1.jpg)

```
输入：root = [1,null,2,3]
输出：[3,2,1]
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

- 树中节点的数目在范围 `[0, 100]` 内
- `-100 <= Node.val <= 100`

 

**进阶：**递归算法很简单，你可以通过迭代算法完成吗？

------

[Discussion](https://leetcode.cn/problems/binary-tree-postorder-traversal/comments/) | [Solution](https://leetcode.cn/problems/binary-tree-postorder-traversal/solution/)

**题解**

**递归思路**

先递归访问左子树，然后递归访问右子树，最后访问根节点，即实现左子树->右子树->根节点的访问

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
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = vec![];
        if root.is_none() {
            return result;
        }

        postorder_recursive(root, &mut result);

        result
    }
}

fn postorder_recursive(root: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
    match root {
        Some(node) => {
            // 递归遍历左子树
            postorder_recursive(node.borrow().left.clone(), result);
            // 递归遍历右子树
            postorder_recursive(node.borrow().right.clone(), result);
            // 访问当前节点
            result.push(node.borrow().val);
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
            Solution::postorder_traversal(tree![1, null, 2, 3]),
            vec![3, 2, 1]
        );
        assert_eq!(Solution::postorder_traversal(tree![]), vec![]);
        assert_eq!(Solution::postorder_traversal(tree![1]), vec![1]);
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

前序遍历的入栈顺序是 根节点 -> 左子树 -> 右子树。根据前序遍历的实现思路，在入栈的过程中，优先入右子树，再入左子树，即将入栈顺序调整为 根节点 -> 右子树 -> 左子树。最后，依次将栈顶的节点出栈就是后序遍历的访问顺序：左子树 -> 右子树 -> 根节点。

我们可以使用 stack1 和 stack2 两个栈来实现上述操作。先将根节点入 stack1，当 stack1 不为空时，重复执行：将栈顶的节点出栈，并将该节点入 stack2，如果该节点有左子节点，则将左子节点入 stack1；如果该节点有右子节点，则将右子节点入 stack1。

直到 stack1 为空，所有节点按根节点 -> 右子树 -> 左子树的顺序放入 stack2。再依次将 stack2 栈顶的节点出栈。即可完成后序遍历。也就是说，根节点是第一个入 stack1，也是第一个出 stack1。其他节点在 stack1 中的入栈顺序是先左后右，出栈顺序是先右后左。在 stack2 中，根节点是第一个入栈，最后一个出栈，其他节点的入栈顺序是先右后左，出栈顺序是先左后右。

非递归实现的算法流程如下：

1、创建 stack1，stack2 两个栈用来存放节点，并先将根节点入 stack1

2、让 stack1 栈顶的节点出栈，将该节点入 stack2，同时将该节点的左右子节点入栈 stack1

3、重复步骤2，直到 stack1 为空

4、将 stack2 栈顶的节点出栈，访问该节点

5、重复步骤 4，直到 stack2 为空，完成所有节点的访问

```rust
impl Solution {
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = vec![];
        if root.is_none() {
            return result;
        }

        let mut stack1 = Vec::new();
        let mut stack2 = Vec::new();
        stack1.push(root);

        // 将 stack1 栈顶的节点依次出栈，并将改节点放入 stack2，将该节点的左右子节点入 stack1
        while let Some(Some(node)) = stack1.pop() {
            if node.borrow().left.is_some() {
                stack1.push(node.borrow().left.clone());
            }
            if node.borrow().right.is_some() {
                stack1.push(node.borrow().right.clone());
            }
            stack2.push(Some(node));
        }

        // 将 stack2 栈顶的节点依次出栈，并访问其节点值
        while let Some(Some(node)) = stack2.pop() {
            result.push(node.borrow().val);
        }
        result
    }

}
```

