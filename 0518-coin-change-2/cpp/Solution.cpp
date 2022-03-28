class Solution {
public:
    int change(int amount, vector<int> &coins) {
        int n = coins.size();
        vector<vector<int>> dp(n, vector<int>(amount + 1));
        for (int c = 0; c <= amount / coins[0]; ++c) {
            dp[0][c * coins[0]] = 1;
        }
        for (int i = 1; i < n; ++i) {
            for (int j = 0; j <= amount; ++j) {
                int k = j / coins[i];
                for (int c = 0; c <= k; ++c) {
                    dp[i][j] += dp[i - 1][j - c * coins[i]];
                }
            }
        }
        return dp[n - 1][amount];
    }
};