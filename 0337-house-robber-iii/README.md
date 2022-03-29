# 337. 打家劫舍 III
在上次打劫完一条街道之后和一圈房屋后，小偷又发现了一个新的可行窃的地区。这个地区只有一个入口，我们称之为“根”。 除了“根”之外，每栋房子有且只有一个“父“房子与之相连。一番侦察之后，聪明的小偷意识到“这个地方的所有房屋的排列类似于一棵二叉树”。 如果两个直接相连的房子在同一天晚上被打劫，房屋将自动报警。

计算在不触动警报的情况下，小偷一晚能够盗取的最高金额。

#### 示例 1:
<pre>
<strong>输入:</strong> [3,2,3,null,3,null,1]

     <b>3</b>
    / \
   2   3
    \   \
     <b>3   1</b>
<strong>输出:</strong> 7
<strong>解释:</strong> 小偷一晚能够盗取的最高金额 = <b>3</b> + <b>3</b> + <b>1</b> = <b>7</b>.
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> [3,4,5,1,3,null,1]

     3
    / \
   <b>4   5</b>
  / \   \
 1   3   1
<strong>输出:</strong> 9
<strong>解释:</strong> 小偷一晚能够盗取的最高金额 = <b>4</b> + <b>5</b> = <b>9</b>.
</pre>

## 题解

### Rust
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
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let money = Self::postorder(root);
        money[0].max(money[1])
    }
    fn postorder(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut money = vec![0; 2];

        if root.is_none() {
            return money;
        }
        let left_money = Self::postorder(root.as_ref().unwrap().borrow().left.clone());
        let right_money = Self::postorder(root.as_ref().unwrap().borrow().right.clone());
        money[0] =
            i32::max(left_money[0], left_money[1]) + i32::max(right_money[0], right_money[1]);
        money[1] = (left_money[0] + right_money[0]) + root.as_ref().unwrap().borrow().val;
        money
    }
}
```

### Go
```go
package main

import "math"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func rob(root *TreeNode) int {
	money := postorder(root)
	return int(math.Max(float64(money[0]), float64(money[1])))
}
func postorder(root *TreeNode) []int {
	if root == nil {
		return []int{0, 0}
	}
	leftMoney := postorder(root.Left)
	rightMoney := postorder(root.Right)
	money := make([]int, 2)
	money[0] = int(math.Max(float64(leftMoney[0]), float64(leftMoney[1]))) + int(math.Max(float64(rightMoney[0]), float64(rightMoney[1])))
	money[1] = (leftMoney[0] + rightMoney[0]) + root.Val
	return money
}

```

### JavaScript
```javascript
/**
 * Definition for a binary tree node.
 * function TreeNode(val, left, right) {
 * this.val = (val===undefined ? 0 : val)
 * this.left = (left===undefined ? null : left)
 * this.right = (right===undefined ? null : right)
 * }
 */
/**
 * @param {TreeNode} root
 * @return {number}
 */
var rob = function(root) {
    let postorder = node => {
        if (node == null) {
            return [0, 0]
        }
        let leftMoney = postorder(node.left)
        let rightMoney = postorder(node.right)
        let money = new Array(2)
        money[0] = Math.max(leftMoney[0], leftMoney[1]) + Math.max(rightMoney[0],
            rightMoney[1])
        money[1] = (leftMoney[0] + rightMoney[0]) + node.val
        return money
    }
    let money = postorder(root)
    return Math.max(money[0], money[1])
};
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
    def rob(self, root: TreeNode) -> int:
        money = self.postorder(root)
        return max(money[0], money[1])

    def postorder(self, root):
        if not root:
            return [0,0]
        leftMoney = self.postorder(root.left)
        rightMoney = self.postorder(root.right)
        money = [0, 0]
        money[0] = max(leftMoney[0], leftMoney[1]) + max(rightMoney[0], rightMoney[1])
        money[1] = leftMoney[0] + rightMoney[0] + root.val
        return money
```

### C++
```c++
class Solution {
public:
    int rob(TreeNode *root) {
        vector<int> money = postorder(root);
        return std::max(money[0], money[1]);
    }
    vector<int> postorder(TreeNode *root) {
        if (root == nullptr) {
            return {0, 0};
        }
        vector<int> leftMoney = postorder(root->left);
        vector<int> rightMoney = postorder(root->right);
        vector<int> money(2);
        money[0] = std::max(leftMoney[0], leftMoney[1]) + std::max(rightMoney[0],
                                                                   rightMoney[1]);
        money[1] = (leftMoney[0] + rightMoney[0]) + root->val;
        return money;
    }
};
```