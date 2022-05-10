# 437. 路径总和 III
给定一个二叉树，它的每个结点都存放着一个整数值。

找出路径和等于给定数值的路径总数。

路径不需要从根节点开始，也不需要在叶子节点结束，但是路径方向必须是向下的（只能从父节点到子节点）。

二叉树不超过1000个节点，且节点数值范围是 [-1000000,1000000] 的整数。

#### 示例:
<pre>
root = [10,5,-3,3,2,null,11,3,-2,null,1], sum = 8

      10
     /  \
    <strong>5   -3</strong>
   <strong>/ \    \</strong>
  <strong>3   2   11</strong>
 / \   <strong>\</strong>
3  -2   <strong>1</strong>

返回 3。和等于 8 的路径有:

1.  5 -> 3
2.  5 -> 2 -> 1
3. -3 -> 11
</pre>

## 题解

### Rust
方法一：
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
use std::collections::HashMap;
use std::rc::Rc;

struct Solution;

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        let mut count = 0;
        Self::dfs(root, target_sum, &mut count);
        count
    }
    // 返回以 root 为上端点的路径的所有的长度 (key) 对应的路径的个数 (value)
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, sum: i32, count: &mut i32) -> HashMap<i32, i32> {
        if root.is_none() {
            return HashMap::new();
        }
        let mut left_values = Self::dfs1(root.as_ref().unwrap().borrow().left.clone(), sum, count);
        let mut right_values =
            Self::dfs1(root.as_ref().unwrap().borrow().right.clone(), sum, count);
        let mut root_values = HashMap::new();
        root_values.insert(root.as_ref().unwrap().borrow().val, 1);

        for (key, val) in left_values.iter_mut() {
            let new_key = root.as_ref().unwrap().borrow().val + key;
            let mut new_value = *val;
            if root_values.contains_key(&new_key) {
                new_value += root_values.get(&new_key).unwrap();
            }
            root_values.insert(new_key, new_value);
        }
        for (key, val) in right_values.iter_mut() {
            let new_key = root.as_ref().unwrap().borrow().val + key;
            let mut new_value = *val;
            if root_values.contains_key(&new_key) {
                new_value += root_values.get(&new_key).unwrap();
            }
            root_values.insert(new_key, new_value);
        }
        for (key, value) in &root_values {
            if *key == sum {
                *count += *value;
            }
        }
        root_values
    }
}
```
方法二：
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

struct Solution;

impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        let mut count = 0;
        Self::dfs(root, &mut vec![], target_sum, &mut count);
        count
    }
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, path: &mut Vec<i32>, sum: i32, count: &mut i32) {
        if root.is_none() {
            return;
        }
        path.push(root.as_ref().unwrap().borrow().val);

        let mut cur_sum = 0;
        for i in (0..=path.len() - 1).rev() {
            cur_sum += path[i];
            if cur_sum == sum {
                *count += 1;
            }
        }

        Self::dfs(
            root.as_ref().unwrap().borrow().left.clone(),
            path,
            sum,
            count,
        );
        Self::dfs(
            root.as_ref().unwrap().borrow().right.clone(),
            path,
            sum,
            count,
        );

        path.pop();
    }
}
```

### Python
```python
# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

class Solution:
    def __init__(self):
        self.count = 0

    def pathSum(self, root: TreeNode, targetSum: int) -> int:
        self.dfs(root, targetSum)
        return self.count

    def dfs(self, root, targetSum):
        if not root:
            return {}
        leftValues = self.dfs(root.left, targetSum)
        rightValues = self.dfs(root.right, targetSum)
        rootValues = dict()
        rootValues[root.val] = 1
        for key, value in leftValues.items():
            key = key + root.val
            if key in rootValues:
                value += rootValues[key]
            rootValues[key] = value
        for key, value in rightValues.items():
            key = key + root.val
            if key in rootValues:
                value += rootValues[key]
            rootValues[key] = value
        for key, value in rootValues.items():
            if key == targetSum:
                self.count += value
        return rootValues


class Solution:
    def __init__(self):
        self.count = 0
    def pathSum(self, root: TreeNode, targetSum: int) -> int:
        self.dfs(root,[], targetSum)
        return self.count
    def dfs(self, root, path, targetSum):
        if not root:
            return
        path.append(root.val)
        curSum = 0
        for i in range(len(path)-1, -1,-1):
            curSum += path[i]
            if curSum == targetSum:
                self.count += 1
        self.dfs(root.left, path, targetSum)
        self.dfs(root.right, path, targetSum)
        path.pop()
```

### C++
方法一：

```c++
class Solution {
public:
    int count = 0;
    int pathSum(TreeNode* root, int sum) {
        dfs(root, sum);
        return count;
    }
    unordered_map<int, int> dfs(TreeNode* root, int sum) {
        unordered_map<int, int> map;
        if (!root) return map;
        unordered_map<int, int> leftValues = dfs(root->left, sum);
        unordered_map<int, int> rightValues = dfs(root->right, sum);
        unordered_map<int, int> rootValues = map;
        rootValues.insert({root->val, 1});
        for (auto it = leftValues.begin(); it != map.end(); ++it) {
            int newKey = it->first + root->val;
            int newValue = it->second;
            if (rootValues.find(newKey) != rootValues.end()) {
                newValue += rootValues[newKey];
            }
            rootValues[newKey] = newValue;
        }
        for (auto it = rightValues.begin(); it != rightValues.end(); ++it) {
            int newKey = it->first + root->val;
            int newValue = it->second;
            if (rootValues.find(newKey) != rootValues.end()) {
                newValue += rootValues[newKey];
            }
            rootValues[newKey] = newValue;
        }
        for (auto it = rootValues.begin(); it != rootValues.end(); ++it) {
            if (it->first == sum) {
                count += it->second;
            }
        }
        return rootValues;
    }
};
```

方法二：

```c++

class Solution {
public:
    int count = 0;
    int pathSum(TreeNode* root, int targetSum) {
        vector<int> path;
        dfs(root, path, targetSum);
        return count;
    }
    void dfs(TreeNode* root, vector<int>& path, int sum) {
        if (root == nullptr) return;
        path.push_back(root->val);
        int curSum = 0;
        for (int i = path.size() - 1; i >= 0; --i) {
            curSum += path[i];
            if (curSum == sum) count++;
        }
        dfs(root->left, path, sum);
        dfs(root->right, path, sum);
        path.pop_back();
    }
};
```