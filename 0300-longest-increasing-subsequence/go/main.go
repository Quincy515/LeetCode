package main

import "math"

func lengthOfLIS(nums []int) int {
	n := len(nums)
	dp := make([]int, n)
	dp[0] = 1
	for i := 1; i < n; i++ {
		dp[i] = 1
		for j := 0; j < i; j++ {
			if nums[i] > nums[j] {
				dp[i] = int(math.Max(float64(dp[i]), float64(dp[j]+1)))
			}
		}
	}
	result := 0
	for i := 0; i < n; i++ {
		if dp[i] > result {
			result = dp[i]
		}
	}
	return result
}

//解法 2
func lengthOfLIS2(nums []int) int {
	n := len(nums)
	lisToMinV := make([]int, n+1)
	k := 0
	dp := make([]int, n)
	for i := 0; i < n; i++ {
		len := bsearch(lisToMinV, k, nums[i])
		if len == -1 {
			dp[i] = 1
		} else {
			dp[i] = len + 1
		}
		if dp[i] > k {
			k = dp[i]
			lisToMinV[dp[i]] = nums[i]
		} else if lisToMinV[dp[i]] > nums[i] {
			lisToMinV[dp[i]] = nums[i]
		}
	}
	result := 0
	for i := 0; i < n; i++ {
		if dp[i] > result {
			result = dp[i]
		}
	}
	return result
}

//查找最后一个比 target 小的元素位置
func bsearch(a []int, k, target int) int {
	low := 1
	high := k
	for low <= high {
		mid := (low + high) / 2
		if a[mid] < target {
			if mid == k || a[mid+1] >= target {
				return mid
			} else {
				low = mid + 1
			}
		} else {
			high = mid - 1
		}
	}
	return -1
}
