## [面试题 04.06. 后继者](https://leetcode.cn/problems/successor-lcci/)

设计一个算法，找出二叉搜索树中指定节点的“下一个”节点（也即中序后继）。

如果指定节点没有对应的“下一个”节点，则返回null。

**示例 1:**

```
输入: root = [2,1,3], p = 1

  2
 / \
1   3

输出: 2
```

**示例 2:**

    输入: root = [5,3,6,2,4,null,null,1], p = 6 
    
          5
         / \
        3   6
       / \
      2   4
     /   
    1
    
    输出: null
**思路**

1、使用递归对一棵树进行中序遍历

2、记录一个 flag 和一个 result，初始化 flag 为 false，ret 为 空，然后中序遍历树，如果发现当前树的根节点等于 p，则 flag = true，如果 flag = true 并且 ret 为空，则将当前节点的根节点赋值给 ret

**题解**

```c++
/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode(int x) : val(x), left(NULL), right(NULL) {}
 * };
 */
class Solution {
    int flag;
    TreeNode* ret;
    void dfs(TreeNode* root, TreeNode* p) {
        if (root == nullptr) {
            return;
        }

        dfs(root->left, p);

        if (flag == true && ret == nullptr) {
            ret = root;
        }
        if (root == p) {
            flag = true;
        }

        dfs(root->right, p);
    }
public:
    TreeNode* inorderSuccessor(TreeNode* root, TreeNode* p) {
        ret = nullptr;
        flag = false;
        dfs(root, p);
        return ret;
    }
};
```

