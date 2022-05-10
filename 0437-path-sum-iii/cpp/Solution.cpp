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