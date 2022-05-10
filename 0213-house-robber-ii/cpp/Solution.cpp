class Solution {
public:
    int rob(vector<int> &nums) {
        int n = nums.size();
        if (n == 1) return nums[0];
        if (n == 2) return std::max(nums[0], nums[1]);
        int max1 = rob_dp(nums, 1, n - 1);
        int max2 = nums[0] + rob_dp(nums, 2, n - 2);
        return std::max(max1, max2);
    }
    int rob_dp(vector<int> nums, int p, int r) {
        int n = nums.size();
        vector<vector<int>> dp(n, vector<int>(2));
        dp[p][0] = 0;
        dp[p][1] = nums[p];
        for (int i = p + 1; i <= r; ++i) {
            dp[i][0] = std::max(dp[i - 1][0], dp[i - 1][1]);
            dp[i][1] = dp[i - 1][0] + nums[i];
        }
        return std::max(dp[r][0], dp[r][1]);
    }
};