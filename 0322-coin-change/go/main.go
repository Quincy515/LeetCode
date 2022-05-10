package main

import "math"

func coinChange(coins []int, amount int) int {
	n := len(coins)
	dp := make([][]int, n)
	for i := 0; i < n; i++ {
		dp[i] = make([]int, amount+1)
		for j := 0; j <= amount; j++ {
			dp[i][j] = math.MaxInt32
		}
	}
	for c := 0; c <= amount/coins[0]; c++ {
		dp[0][c*coins[0]] = c
	}
	for i := 1; i < n; i++ {
		for j := 0; j <= amount; j++ {
			k := j / coins[i]
			for c := 0; c <= k; c++ {
				if dp[i-1][j-c*coins[i]] != math.MinInt32 && dp[i-1][j-c*coins[i]]+c < dp[i][j] {
					dp[i][j] = dp[i-1][j-c*coins[i]] + c
				}
			}
		}
	}
	if dp[n-1][amount] == math.MaxInt32 {
		return -1
	}
	return dp[n-1][amount]
}

func coinChange(coins []int, amount int) int {
	k := len(coins)
	dp := make([]int, amount+1)
	for i := 0; i <= amount; i++ {
		dp[i] = math.MaxInt32
	}
	dp[0] = 0
	for i := 1; i <= amount; i++ {
		for j := 0; j < k; j++ {
			if i-coins[j] >= 0 && dp[i-coins[j]] != math.MaxInt32 && dp[i-coins[j]]+1 < dp[i] {
				dp[i] = dp[i-coins[j]] + 1
			}
		}
	}
	if dp[amount] == math.MaxInt32 {
		return -1
	}
	return dp[amount]
}