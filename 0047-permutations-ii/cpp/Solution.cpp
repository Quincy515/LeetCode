class Solution {
public:
    vector<vector<int>> result;
    vector<vector<int>> permuteUnique(vector<int>& nums) {
        unordered_map<int, int> hm;
        for (int i = 0; i < nums.size(); ++i) {
            int count = 1;
            auto it = hm.find(nums[i]);
            if (it != hm.end()) {
                count += hm[nums[i]];
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
                counts[k] = hm[nums[i]];
                k++;
                hm.erase(it);:
            }
        }
        vector<int> path;
        backtrack(uniqueNums, counts, 0, path, nums.size());
        return result;
    }
    void backtrack(vector<int> uniqueNums, vector<int> counts, int k, vector<int> &path, int n) {
        if (k == n) {
            result.push_back(path);
            return;
        }
        for (int i = 0; i < uniqueNums.size(); ++i) {
            if (counts[i] == 0) continue;
            path.push_back(uniqueNums[i]);
            counts[i]--;
            backtrack(uniqueNums, counts, k + 1, path, n);
            path.pop_back();
            counts[i]++;
        }
    }
};