class Solution {
public:
    vector<vector<int>> result;
    vector<vector<int>> permute(vector<int>& nums) {
        vector<int> path;
        backtrack(nums, 0, path);
        return result;
    }
    void backtrack(vector<int> nums, int k, vector<int> &path) {
        if (k == nums.size()) {
            result.push_back(path);
        }
        for (int i = 0; i < nums.size(); ++i) {
            if (find(path.begin(), path.end(), nums[i]) != path.end()) {
                continue;
            }
            path.push_back(nums[i]);
            backtrack(nums, k + 1, path);
            path.pop_back();
        }
    }
};