class Solution {
public:
    int cuttingRope(int n) {
        if (n == 1) return 1;
        if (n == 2) return 1;
        if (n == 3) return 2;
        std::vector<int> dp(n + 1);
        dp[0] = 1;
        for (int i = 1; i <= n; ++i) {
            for (int j = 1; j <= i; ++j) {
                if (dp[i] < j * dp[i - j]) {
                    dp[i] = j * dp[i - j];
                }
            }
        }
        return dp[n];
    }
};