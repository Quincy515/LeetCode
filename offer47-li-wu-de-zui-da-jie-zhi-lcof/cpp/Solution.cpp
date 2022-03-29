class Solution {
public:
    int maxValue(vector <vector<int>> &grid) {
        int n = grid.size();
        int m = grid[0].size();
        vector<vector<int>> dp(n, vector<int>(m));
        int sum = 0;
        for (int j = 0; j < m; ++j) {
            sum += grid[0][j];
            dp[0][j] = sum;
        }
        sum = 0;
        for (int i = 0; i < n; ++i) {
            sum += grid[i][0];
            dp[i][0] = sum;
        }
        for (int i = 1; i < n; ++i) {
            for (int j = 1; j < m; ++j) {
                dp[i][j] = std::max(dp[i - 1][j], dp[i][j - 1]) + grid[i][j];
            }
        }
        return dp[n - 1][m - 1];
    }
};