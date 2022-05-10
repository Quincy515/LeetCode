class Solution:
    def findTargetSumWays(self, nums: List[int], target: int) -> int:
        if target > 1000 or target < -1000:
            return 0
        n = len(nums)
        offset = 1000
        w = 2000
        dp = [[0] * (w + 1) for i in range(n)]
        dp[0][offset - nums[0]] += 1
        dp[0][offset + nums[0]] += 1
        for i in range(1, n):
            for j in range(w+1):
                if 0 <= j - nums[i] <= w:
                    dp[i][j] = dp[i-1][j-nums[i]]
                if 0 <= j + nums[i] <= w:
                    dp[i][j] += dp[i-1][j+nums[i]]
        return dp[n-1][target+offset]
