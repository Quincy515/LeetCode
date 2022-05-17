#### [102. 二叉树的层序遍历](https://leetcode.cn/problems/binary-tree-level-order-traversal/)

给你二叉树的根节点 `root` ，返回其节点值的 **层序遍历** 。 （即逐层地，从左到右访问所有节点）。



**示例 1：**

![img](https://assets.leetcode.com/uploads/2021/02/19/tree1.jpg)

```
输入：root = [3,9,20,null,null,15,7]
输出：[[3],[9,20],[15,7]]
```

**示例 2：**

```
输入：root = [1]
输出：[[1]]
```

**示例 3：**

```
输入：root = []
输出：[]
```



**提示：**

- 树中节点数目在范围 `[0, 2000]` 内
- `-1000 <= Node.val <= 1000`

**思路**

1、首先将根节点放入队列，入队过程记录深度

2、出队过程，根据深度来选择放入结果数组的哪一层

3、剩下的就是广度优先搜索的模板

**题解**

模板代码背诵

```rust
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
```

