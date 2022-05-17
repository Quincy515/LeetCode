#### [LCP 44. 开幕式焰火](https://leetcode.cn/problems/sZ59z6/)

「力扣挑战赛」开幕式开始了，空中绽放了一颗二叉树形的巨型焰火。
给定一棵二叉树 `root` 代表焰火，节点值表示巨型焰火这一位置的颜色种类。请帮小扣计算巨型焰火有多少种不同的颜色。

**示例 1：**

> 输入：`root = [1,3,2,1,null,2]`
>
> 输出：`3`
>
> 解释：焰火中有 3 个不同的颜色，值分别为 1、2、3

**示例 2：**

> 输入：`root = [3,3,3]`
>
> 输出：`1`
>
> 解释：焰火中仅出现 1 个颜色，值为 3

**提示：**

- `1 <= 节点个数 <= 1000`
- `1 <= Node.val <= 1000`

**思路**

1、需要一个哈希表和一个队列

2、将根节点塞入队列，利用广度优先搜索扩展状态，对每个扩展出来的节点放入哈希表，放入之前统计是否出现过从而计数

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
    pub fn num_color(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut result = 0;
        let mut hash = std::collections::HashMap::new();
        let mut q = std::collections::VecDeque::new();
        q.push_back(root.clone());

        while !q.is_empty() {
            if let Some(now) = q.pop_front() {
                if now.is_none() {
                    continue;
                }
                if !hash.contains_key(&(now.as_ref().unwrap().borrow().val)) {
                    hash.insert(now.as_ref().unwrap().borrow().val, 1);
                    result += 1;
                }
                q.push_back(now.as_ref().unwrap().borrow().left.clone());
                q.push_back(now.as_ref().unwrap().borrow().right.clone());
            }
        }

        result
    }
}
struct Solution;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::num_color(tree![1, 3, 2, 1, null, 2]), 3);
        assert_eq!(Solution::num_color(tree![3, 3, 3]), 1);
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

