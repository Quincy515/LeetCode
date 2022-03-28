package main

func findTargetSumWays(nums []int, target int) int {
	if target > 1000 || target < -1000 {
		return 0
	}
	n := len(nums)
	offset := 1000
	w := 2000
	dp := make([][]int, n)
	for i := 0; i < len(dp); i++ {
		dp[i] = make([]int, w+1)
	}
	dp[0][offset-nums[0]] += 1
	dp[0][offset+nums[0]] += 1
	for i := 1; i < n; i++ {
		for j := 0; j <= w; j++ {
			if j-nums[i] >= 0 && j-nums[i] <= w {
				dp[i][j] = dp[i-1][j-nums[i]]
			}
			if j+nums[i] >= 0 && j+nums[i] <= w {
				dp[i][j] += dp[i-1][j+nums[i]]
			}
		}
	}
	return dp[n-1][target+1000]
}
