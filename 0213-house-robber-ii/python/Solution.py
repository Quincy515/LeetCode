class Solution:
    def rob(self, nums: List[int]) -> int:
        n = len(nums)
        if n == 1:
            return nums[0]
        if n == 2:
            return max(nums[0], nums[1])
        max1 = self.rob_dp(nums, 1,n-1)
        max2 = self.rob_dp(nums, 2,n-2) + nums[0]
        return max(max1, max2)

    def rob_dp(self,nums,p,r):
        n = len(nums)
        dp = [[0,0] for _ in range(n)]
        dp[p][0] = 0
        dp[p][1] = nums[p]
        for i in range(p+1, r+1):
            dp[i][0] = max(dp[i-1][0], dp[i-1][1])
            dp[i][1] = dp[i-1][0] + nums[i]
        return max(dp[r][0], dp[r][1])
