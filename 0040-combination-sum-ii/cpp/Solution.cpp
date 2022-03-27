class Solution {
public:
    vector<vector<int>> result;
    vector<vector<int>> combinationSum2(vector<int>& candidates, int target) {
        unordered_map<int, int> hashTable;
        for (int i = 0; i < candidates.size(); ++i) {
            auto it = hashTable.find(candidates[i]);
            if (it == hashTable.end()) {
                hashTable[candidates[i]] = 1;
            } else {
                hashTable[candidates[i]] = hashTable[candidates[i]] + 1;
            }
        }
        vector<int> nums;
        vector<int> counts;
        for (int i = 0; i < candidates.size(); ++i) {
            auto it = hashTable.find(candidates[i]);
            if (it != hashTable.end()) {
                nums.push_back(candidates[i]);
                counts.push_back(hashTable[candidates[i]]);
                hashTable.erase(it);
            }
        }
        vector<int> path;
        backtrack(nums, counts, 0, target, path);
        return result;
    }
    void backtrack(vector<int> nums, vector<int> counts, int k, int left, vector<int> &path) {
        if (left == 0) {
            result.push_back(path);
            return;
        }
        if (left < 0 || k == nums.size()) {
            return;
        }
        for (int count = 0; count <= counts[k]; ++count) {
            for (int i = 0; i < count; ++i) {
                path.push_back(nums[k]);
            }
            backtrack(nums, counts, k + 1, left - count * nums[k], path);
            for (int i = 0; i < count; ++i) {
                path.pop_back();
            }
        }
    }
};