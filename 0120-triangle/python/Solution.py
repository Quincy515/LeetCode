class Solution:
    def minimumTotal(self, triangle: List[List[int]]) -> int:
        n = len(triangle)
        dp = [[float("inf")] * n for _ in range(n)]
        dp[0][0] = triangle[0][0]
        for i in range(1,n):
            dp[i][0] = dp[i-1][0] + triangle[i][0]
            for j in range(1,i):
                dp[i][j] = min(dp[i-1][j],dp[i-1][j-1]) + triangle[i][j]
                dp[i][i] = dp[i-1][i-1] + triangle[i][i]
        res = float("inf")
        for j in range(n):
            if dp[n-1][j] < res:
                res = dp[n-1][j]
        return res
