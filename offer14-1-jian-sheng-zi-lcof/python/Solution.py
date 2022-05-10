class Solution:
    def cuttingRope(self, n: int) -> int:
        if n == 1:
            return 1
        if n == 2:
            return 1
        if n == 3:
            return 2
        dp = [0] * (n +1)
        dp[0] = 1
        for i in range(1,n+1):
            for j in range(1,i+1):
                if dp[i] < dp[i-j] * j:
                   dp[i] = j * dp[i-j]
        return dp[n]