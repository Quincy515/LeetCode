package main

func missingNumber(nums []int) int {
	n := len(nums)
	ret := 0
	for i := 0; i <= n; i++ {
		ret ^= i
	}
	for i := 0; i < n; i++ {
		ret ^= nums[i]
	}
	return ret
}