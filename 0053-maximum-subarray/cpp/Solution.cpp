class Solution {
public:
    int maxSubArray(vector<int>& nums) {
        int n = nums.size();
        int maxSum = INT_MIN;
        int sum = 0;
        for (int i = 0; i < n; ++i) {
            if (sum < 0) {
                sum = 0;
            }
            sum += nums[i];
            if (sum > maxSum) {
                maxSum = sum;
            }
        }
        return maxSum;
    }
};

class Solution {
public:
    int maxSubArray(vector<int>& nums) {
        if (nums.size() == 1) return nums[0];
        vector<int> sum(nums.size());
        vector<int> max(nums.size());
        int cursum = 0;
        for (int i = 0; i < nums.size(); ++i) {
            cursum += nums[i];
            sum[i] = cursum;
        }
        int curmax = INT_MIN;
        for (int i = sum.size() - 1; i >= 0; --i) {
            if (curmax < sum[i]) curmax = sum[i];
            max[i] = curmax;
        }
        int result = INT_MIN;
        for (int i = 0; i < nums.size(); ++i) {
            if (i == 0 && result < max[0]) {
                result = max[0];
            }
            if (i != 0 && result < max[i] - sum[i - 1]) {
                result = max[i] - sum[i - 1];
            }
        }
        return result;
    }
};