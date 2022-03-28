package main

func canPartition(nums []int) bool {
	n := len(nums)
	sum := 0
	for i := 0; i < n; i++ {
		sum += nums[i]
	}
	if sum%2 == 1 {
		return false
	}
	sum /= 2
	dp := make([][]bool, n)
	for i := 0; i < len(dp); i++ {
		dp[i] = make([]bool, sum+1)
	}
	dp[0][0] = true
	if nums[0] <= sum {
		dp[0][nums[0]] = true
	}
	for i := 1; i < n; i++ {
		for j := 0; j <= sum; j++ {
			if j-nums[i] >= 0 {
				dp[i][j] = dp[i-1][j] || dp[i-1][j-nums[i]]
			} else {
				dp[i][j] = dp[i-1][j]
			}
		}
	}
	return dp[n-1][sum]
}
