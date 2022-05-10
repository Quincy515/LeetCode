class Solution {
public:
    int maxProfit(vector<int>& prices) {
        if (prices.size() == 0) return 0;
        int n = prices.size();
        vector<vector<int>> dp(n, vector<int>(4));
        dp[0][0] = -prices[0];
        dp[0][1] = 0;
        dp[0][2] = 0;
        dp[0][3] = 0;
        for (int i = 1; i < n; ++i) {
            dp[i][0] = max3(dp[i-1][0], dp[i-1][2] - prices[i], dp[i - 1][3] -
                                                                prices[i]);
            dp[i][1] = dp[i - 1][0] + prices[i];
            dp[i][2] = dp[i - 1][1];
            dp[i][3] = std::max(dp[i-1][2], dp[i-1][3]);
        }
        return max4(dp[n-1][0], dp[n-1][1], dp[n-1][2], dp[n-1][3]);
    }
    int max3(int a, int b, int c) {
        return std::max(std::max(a, b), c);
    }
    int max4(int a, int b, int c, int d) {
        return std::max(std::max(std::max(a, b), c), d);
    }
};