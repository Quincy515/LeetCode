class Solution {
public:
    vector<vector<int>> result;
    vector<vector<int>> subsets(vector<int>& nums) {
        vector<int> path;
        backtrack(nums, 0, path);
        return result;
    }
    void backtrack(vector<int> nums, int k, vector<int>& path) {
        if (k == nums.size()) {
            // 批注：C++不需要snapshot，push_back()内部会拷⻉⼀份数据的副本
            result.push_back(path);
            return;
        }
        backtrack(nums, k + 1, path);
        path.push_back(nums[k]);
        backtrack(nums, k + 1, path);
        path.pop_back();
    }
};