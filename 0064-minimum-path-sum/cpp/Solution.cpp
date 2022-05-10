class Solution {
public:
    int minPathSum(vector <vector<int>> &grid) {
        int m = grid.size();
        int n = grid[0].size();
        vector<vector<int>> dp(m, vector<int>(n));
        int len = 0;
        for (int i = 0; i < m; ++i) {
            len += grid[i][0];
            dp[i][0] = len;
        }
        len = 0;
        for (int j = 0; j < n; ++j) {
            len += grid[0][j];
            dp[0][j] = len;
        }
        for (int i = 1; i < m; ++i) {
            for (int j = 1; j < n; ++j) {
                dp[i][j] = std::min(dp[i - 1][j], dp[i][j - 1]) + grid[i][j];
            }
        }
        return dp[m - 1][n - 1];
    }
};