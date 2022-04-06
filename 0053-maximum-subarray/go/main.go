package main

import "math"

//滑动窗口
func maxSubArray(nums []int) int {
	n := len(nums)
	maxSum := math.MinInt32
	sum := 0
	for i := 0; i < n; i++ {
		if sum < 0 {
			sum = 0
		}
		sum += nums[i]
		if sum > maxSum {
			maxSum = sum
		}
	}
	return maxSum
} //前后缀
func maxSubArray(nums []int) int {
	if len(nums) == 1 {
		return nums[0]
	}
	sum := make([]int, len(nums))
	max := make([]int, len(nums))
	cursum := 0
	for i := 0; i < len(nums); i++ {
		cursum += nums[i]
		sum[i] = cursum
	}
	curmax := math.MinInt32
	for i := len(sum) - 1; i >= 0; i-- {
		if curmax < sum[i] {
			curmax = sum[i]
		}
		max[i] = curmax
	}
	result := math.MinInt32
	for i := 0; i < len(nums); i++ {
		if i == 0 && result < max[0] {
			result = max[0]
		}
		if i != 0 && result < max[i]-sum[i-1] {
			result = max[i] - sum[i-1]
		}
	}
	return result
}
