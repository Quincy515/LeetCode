class Solution:
    def coinChange(self, coins: List[int], amount: int) -> int:
        n = len(coins)
        dp = [[float('inf')] * (amount+1) for i in range(n)]
        for c in range(amount//coins[0] + 1):
            dp[0][c * coins[0]] = c

        for i in range(1, n):
            for j in range(amount+1):
                k = j // coins[i]
                for c in range(k+1):
                    if dp[i-1][j - c * coins[i]] != float('inf') and dp[i-1][j - c * coins[i]] + c < dp[i][j]:
                        dp[i][j] = dp[i-1][j - c * coins[i]] + c

        if dp[n-1][amount] == float('inf'):
            return -1
        return dp[n-1][amount]

class Solution:
    def coinChange(self, coins: List[int], amount: int) -> int:
        k = len(coins)
        dp = [float("inf")] * (amount+1)
        dp[0] = 0
        for i in range(1, amount+1):
            for j in range(k):
                if i - coins[j] >=0 and dp[i- coins[j]] != float("inf") and dp[i-coins[j]] + 1 < dp[i]:
                    dp[i] = dp[i - coins[j]] + 1
        if dp[amount] == float("inf"):
            return -1
        return dp[amount]
