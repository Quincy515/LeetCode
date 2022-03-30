class Solution {
public:
    int climbStairs(int n) {
        if (n <= 2) return n;
        std::vector<int> dp(n+1);
        dp[1] = 1;
        dp[2] = 2;
        for (int i = 3; i <= n; ++i) {
            dp[i] = dp[i-1] + dp[i-2];
        }
        return dp[n];
    }
};

class Solution {
public:
    int climbStairs(int n) {
        std::vector<int> dp(n+1);
        dp[0] = 1;
        for (int i = 1; i <= n; ++i) {
            if (i - 1 >= 0) dp[i] += dp[i - 1];
            if (i - 2 >= 0) dp[i] += dp[i - 2];
        }
        return dp[n];
    }
};