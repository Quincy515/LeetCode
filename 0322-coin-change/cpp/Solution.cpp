class Solution {
public:
    int coinChange(vector<int> &coins, int amount) {
        int n = coins.size();
        vector <vector<int>> dp(n, vector<int>(amount + 1));
        for (int i = 0; i < n; ++i) {
            for (int j = 0; j <= amount; ++j) {
                dp[i][j] = INT_MAX;
            }
        }
        for (int c = 0; c <= amount / coins[0]; ++c) {
            dp[0][c * coins[0]] = c;
        }
        for (int i = 1; i < n; ++i) {
            for (int j = 0; j <= amount; ++j) {
                int k = j / coins[i];
                for (int c = 0; c <= k; ++c) {
                    if (dp[i - 1][j - c * coins[i]] != INT_MAX &&
                        dp[i - 1][j - c * coins[i]] + c < dp[i][j]) {
                        dp[i][j] = dp[i - 1][j - c * coins[i]] + c;
                    }
                }
            }
        }
        if (dp[n - 1][amount] == INT_MAX) return -1;
        return dp[n - 1][amount];
    }
};