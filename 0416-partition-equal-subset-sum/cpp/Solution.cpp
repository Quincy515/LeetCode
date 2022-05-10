class Solution {
public:
    bool canPartition(vector<int> &nums) {
        int n = nums.size();
        int sum = 0;
        for (int i = 0; i < n; ++i) {
            sum += nums[i];
        }
        if (sum % 2 == 1) return false;
        sum /= 2;
        vector <vector<bool>> dp(n, vector<bool>(sum + 1));
        dp[0][0] = true;
        if (nums[0] <= sum) {
            dp[0][nums[0]] = true;
        }
        for (int i = 1; i < n; ++i) {
            for (int j = 0; j <= sum; ++j) {
                if (j - nums[i] >= 0) {
                    dp[i][j] = dp[i - 1][j] || dp[i - 1][j - nums[i]];
                } else {
                    dp[i][j] = dp[i - 1][j];
                }
            }
        }
        return dp[n - 1][sum];
    }
};