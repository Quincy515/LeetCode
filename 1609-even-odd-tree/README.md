## [1609. 奇偶树](https://leetcode.cn/problems/even-odd-tree/)

如果一棵二叉树满足下述几个条件，则可以称为 **奇偶树** ：

- 二叉树根节点所在层下标为 `0` ，根的子节点所在层下标为 `1` ，根的孙节点所在层下标为 `2` ，依此类推。
- **偶数下标** 层上的所有节点的值都是 **奇** 整数，从左到右按顺序 **严格递增**
- **奇数下标** 层上的所有节点的值都是 **偶** 整数，从左到右按顺序 **严格递减**

给你二叉树的根节点，如果二叉树为 **奇偶树** ，则返回 `true` ，否则返回 `false` 。



**示例 1：**

**![img](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2020/10/04/sample_1_1966.png)**

```
输入：root = [1,10,4,3,null,7,9,12,8,6,null,null,2]
输出：true
解释：每一层的节点值分别是：
0 层：[1]
1 层：[10,4]
2 层：[3,7,9]
3 层：[12,8,6,2]
由于 0 层和 2 层上的节点值都是奇数且严格递增，而 1 层和 3 层上的节点值都是偶数且严格递减，因此这是一棵奇偶树。
```

**示例 2：**

**![img](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2020/10/04/sample_2_1966.png)**

```
输入：root = [5,4,2,3,3,7]
输出：false
解释：每一层的节点值分别是：
0 层：[5]
1 层：[4,2]
2 层：[3,3,7]
2 层上的节点值不满足严格递增的条件，所以这不是一棵奇偶树。
```

**示例 3：**

![img](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2020/10/04/sample_1_333_1966.png)

```
输入：root = [5,9,1,3,5,7]
输出：false
解释：1 层上的节点值应为偶数。
```

**示例 4：**

```
输入：root = [1]
输出：true
```

**示例 5：**

```
输入：root = [11,8,6,1,3,9,11,30,20,18,16,12,10,4,2,17]
输出：true
```



**提示：**

- 树中节点数在范围 `[1, 105]` 内
- `1 <= Node.val <= 106`

**思路**

1、利用[二叉树的层数遍历](https://github.com/CusterFun/LeetCode/tree/main/0102-binary-tree-level-order-traversal)返回二维数组

2、然后对二维数组进行题目中要求的判定

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
    pub fn is_even_odd_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let level_node = Self::level_order(root);
        for i in 0..level_node.len() {
            if (i & 1) != 0 {
                // 偶数，严格递减
                for j in 0..level_node[i].len() {
                    if (level_node[i][j] & 1) != 0 {
                        return false;
                    }
                    if j > 0 && level_node[i][j] >= level_node[i][j - 1] {
                        return false;
                    }
                }
            } else {
                // 奇数，单调递增
                for j in 0..level_node[i].len() {
                    if (level_node[i][j] & 1) == 0 {
                        return false;
                    }
                    if j > 0 && level_node[i][j] <= level_node[i][j - 1] {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut levels = vec![];
        if root.is_none() {
            return levels;
        }

        let mut deque = std::collections::VecDeque::new();
        deque.push_back(root);

        while !deque.is_empty() {
            // 开始当前层
            let mut current_level = vec![];

            // 当前层中的元素个数
            let level_length = deque.len();
            for _ in 0..level_length {
                let n = deque.pop_front();
                if let Some(Some(node)) = n {
                    // 添加当前节点
                    current_level.push(node.borrow().val);

                    // 将当前节点的左、右子节点加入队列
                    if node.borrow().left.is_some() {
                        deque.push_back(node.borrow().left.clone());
                    }
                    if node.borrow().right.is_some() {
                        deque.push_back(node.borrow().right.clone());
                    }
                }
            }
            levels.push(current_level);
        }
        levels
    }
}
struct Solution;
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert!(Solution::is_even_odd_tree(tree![
            1, 10, 4, 3, null, 7, 9, 12, 8, 6, null, null, 2
        ]));
        assert!(!Solution::is_even_odd_tree(tree![5, 4, 2, 3, 3, 7]));
        assert!(!Solution::is_even_odd_tree(tree![5, 9, 1, 3, 5, 7]));
        assert!(Solution::is_even_odd_tree(tree![1]));
        assert!(Solution::is_even_odd_tree(tree![
            11, 8, 6, 1, 3, 9, 11, 30, 20, 18, 16, 12, 10, 4, 2, 17
        ]));
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

