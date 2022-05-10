// 排序+双指针
// 时间复杂度O(nlogn)，空间复杂度O(1)
class Solution {
public:
    vector<vector<int>> pairSums(vector<int> &nums, int target) {
        vector<vector<int>> results;
        if (nums.size() == 0) return result;
        std::sort(nums.begin(), nums.end());
        int i = 0;
        int j = nums.size() - 1;
        while (i < j) {
            if (nums[i] + nums[j] == target) {
                results.push_back({nums[i], nums[j]});
                i++;
                j--;
            } else if (nums[i] + nums[j] < target) {
                i++;
            } else {
                j--;
            }
        }
        return results;
    }
};
