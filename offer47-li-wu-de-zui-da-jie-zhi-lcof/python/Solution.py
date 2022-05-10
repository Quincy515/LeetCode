class Solution:
    def maxValue(self, grid: List[List[int]]) -> int:
        n = len(grid)
        m = len(grid[0])
        dp = [[0] * m for _ in range(n)]
        total = 0
        for i in range(n):
            total += grid[i][0]
            dp[i][0] = total
        total = 0
        for j in range(m):
            total += grid[0][j]
            dp[0][j] = total
        for i in range(1, n):
            for j in range(1, m):
                dp[i][j] = max(dp[i-1][j], dp[i][j-1]) + grid[i][j]
        return dp[n-1][m-1]
