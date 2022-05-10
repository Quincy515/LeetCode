class Solution:
    def minPathSum(self, grid: List[List[int]]) -> int:
        m = len(grid)
        n = len(grid[0])
        dp = [[0] * n for i in range(m)]
        step = 0
        for i in range(m):
            step += grid[i][0]
            dp[i][0] = step
        step = 0
        for j in range(n):
            step += grid[0][j]
            dp[0][j] = step
        for i in range(1,m):
            for j in range(1, n):
                dp[i][j] = min(dp[i-1][j], dp[i][j-1]) + grid[i][j]
        return dp[m-1][n-1]
