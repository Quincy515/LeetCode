class Solution:
    def change(self, amount: int, coins: List[int]) -> int:
        n = len(coins)
        dp = [[0] * (amount + 1) for _ in range(n)]
        for c in range(amount // coins[0] + 1):
            dp[0][c*coins[0]] = 1
        for i in range(1, n):
            for j in range(amount+1):
                k = j // coins[i] + 1
                for c in range(k):
                    dp[i][j] += dp[i-1][j - c * coins[i]]
        return dp[n-1][amount]
