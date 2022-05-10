class Solution {
public:
    int findTargetSumWays(vector<int> &nums, int target) {
        if (target > 1000 || target < -1000) return 0;
        int n = nums.size();
        int offset = 1000;
        int w = 2000;
        vector<vector<int>> dp(n, vector<int>(w + 1));
        dp[0][offset - nums[0]] += 1;
        dp[0][offset + nums[0]] += 1;
        for (int i = 1; i < n; ++i) {
            for (int j = 0; j <= w; ++j) {
                if (j - nums[i] >= 0 && j - nums[i] <= w) {
                    dp[i][j] = dp[i - 1][j - nums[i]];
                }
                if (j + nums[i] >= 0 && j + nums[i] <= w) {
                    dp[i][j] += dp[i - 1][j + nums[i]];
                }
            }
        }
        return dp[n - 1][target + 1000];
    }
};