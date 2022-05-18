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