class Solution {
public:
    vector<vector<int>> result;
    vector<vector<int>> combinationSum(vector<int>& candidates, int target) {
        vector<int> path;
        backtrack(candidates, 0, target, path);
        return result;
    }
    void backtrack(vector<int> candidates, int k, int left, vector<int> &path) {
        if (left == 0) {
            result.push_back(path);
            return;
        }
        if (k == candidates.size()) {
            return;
        }
        for (int i = 0; i <= left / candidates[k]; ++i) {
            for (int j = 0; j < i; ++j) {
                path.push_back(candidates[k]);
            }
            backtrack(candidates, k + 1, left - i * candidates[k], path);
            for (int j = 0; j < i; ++j) {
                path.pop_back();
            }
        }
    }
};