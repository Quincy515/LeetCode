## [1110. 删点成林](https://leetcode.cn/problems/delete-nodes-and-return-forest/)

给出二叉树的根节点 `root`，树上每个节点都有一个不同的值。

如果节点值在 `to_delete` 中出现，我们就把该节点从树上删去，最后得到一个森林（一些不相交的树构成的集合）。

返回森林中的每棵树。你可以按任意顺序组织答案。



**示例 1：**

**![img](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2019/07/05/screen-shot-2019-07-01-at-53836-pm.png)**

```
输入：root = [1,2,3,4,5,6,7], to_delete = [3,5]
输出：[[1,2,null,4],[6],[7]]
```

**示例 2：**

```
输入：root = [1,2,4,null,3], to_delete = [3]
输出：[[1,2,4]]
```



**提示：**

- 树中的节点数最大为 `1000`。
- 每个节点都有一个介于 `1` 到 `1000` 之间的值，且各不相同。
- `to_delete.length <= 1000`
- `to_delete` 包含一些从 `1` 到 `1000`、各不相同的值。

**题解**

```rust
impl Solution {
    pub fn del_nodes(
        root: Option<Rc<RefCell<TreeNode>>>,
        to_delete: Vec<i32>,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut result = vec![];
        Solution::dfs(&root, true, &mut result, &to_delete);
        result
    }
    fn dfs(
        root: &Option<Rc<RefCell<TreeNode>>>,
        is_root: bool,
        result: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
        to_delete: &[i32],
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            None => None,
            Some(x) => {
                let mut node = x.borrow_mut();
                let val = node.val;

                let mut flag = false;
                for &i in to_delete {
                    if i == val {
                        flag = true;
                        break;
                    }
                }
                if !flag && is_root {
                    result.push(Some(x.clone()));
                }
                node.left = Self::dfs(&node.left, flag, result, to_delete);
                node.right = Self::dfs(&node.right, flag, result, to_delete);
                if flag {
                    None
                } else {
                    root.clone()
                }
            }
        }
    }
}
```

