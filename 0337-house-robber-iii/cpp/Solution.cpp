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