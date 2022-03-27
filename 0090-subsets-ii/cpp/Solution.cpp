class Solution {
public:
    vector<vector<int>> result;
    vector<vector<int>> subsetsWithDup(vector<int>& nums) {
        unordered_map<int, int> hm;
        for (int i = 0; i < nums.size(); ++i) {
            int count = 1;
            auto it = hm.find(nums[i]);
            if (it != hm.end()) {
                count += it->second;
            }
            hm[nums[i]] = count;
        }
        int n = hm.size();
        vector<int> uniqueNums(n);
        vector<int> counts(n);
        int k = 0;
        for (int i = 0; i < nums.size(); ++i) {
            auto it = hm.find(nums[i]);
            if (it != hm.end()) {
                uniqueNums[k] = nums[i];
                counts[k] = it->second;
                k++;
                hm.erase(it);
            }
        }
        vector<int> path;
        backtrack(uniqueNums, counts, 0, path);
        return result;
    }
    void backtrack(vector<int> uniqueNums, vector<int> counts, int k, vector<int>&
    path) {
        if (k == uniqueNums.size()) {
            result.push_back(path);
            return;
        }
        for (int count = 0; count <= counts[k]; ++count) {
            for (int i = 0; i < count; ++i) {
                path.push_back(uniqueNums[k]);
            }
            backtrack(uniqueNums, counts, k + 1, path);
            for (int i = 0; i < count; ++i) {
                path.pop_back();
            }
        }
    }
};