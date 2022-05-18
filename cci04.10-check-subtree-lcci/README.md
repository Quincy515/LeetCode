## [面试题 04.10. 检查子树](https://leetcode.cn/problems/check-subtree-lcci/)

检查子树。你有两棵非常大的二叉树：T1，有几万个节点；T2，有几万个节点。设计一个算法，判断 T2 是否为 T1 的子树。

如果 T1 有这么一个节点 n，其子树与 T2 一模一样，则 T2 为 T1 的子树，也就是说，从节点 n 处把树砍断，得到的树与 T2 完全相同。

**注意：**此题相对书上原题略有改动。

**示例1:**

```
 输入：t1 = [1, 2, 3], t2 = [2]
 输出：true
```

**示例2:**

```
 输入：t1 = [1, null, 2, 4], t2 = [3, 2]
 输出：false
```



**提示：**

树的节点数目范围为[0, 20000]。


**思路**

带空节点的前序遍历都可以唯一确定一棵二叉树，要判断一棵树是否包含另一棵树，我们可以分别遍历两棵树生成字符串，对比其中一个字符串是否包含另一个字符串即可；

时间复杂度为 O(n+m) ，空间复杂度为 O(n+m)，其中 n 和 m 分别是 t1 和 t2 中节点的数目。

**题解**

```rust
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn check_sub_tree(
        t1: Option<Rc<RefCell<TreeNode>>>,
        t2: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        let mut t1_str = String::new();
        Self::preorder_recursive(t1, &mut t1_str);
        let mut t2_str = String::new();
        Self::preorder_recursive(t2, &mut t2_str);

        t1_str.contains(&t2_str)
    }
    fn preorder_recursive(root: Option<Rc<RefCell<TreeNode>>>, result: &mut String) {
        match root {
            Some(node) => {
                // 访问当前节点
                result.push_str(&node.borrow().val.to_string());
                // 递归遍历左子树
                Self::preorder_recursive(node.borrow().left.clone(), result);
                // 递归遍历右子树
                Self::preorder_recursive(node.borrow().right.clone(), result);
            }
            None => {
                result.push('x');
            }
        }
    }
}
```

