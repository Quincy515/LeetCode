package main

func maxSlidingWindow3(nums []int, k int) []int {
	if nums == nil || len(nums) == 0 {
		return nums
	}
	window := make([]int, 0, k) // store the index of nums
	result := make([]int, 0, len(nums)-k+1)
	for i, v := range nums { // if the left-most index is out of window, remove it
		if i >= k && window[0] <= i-k {
			window = window[1:] // 移除最左边的值
		}
		for len(window) > 0 && nums[window[len(window)-1]] < v { // maintain window
			window = window[:len(window)-1]
		}
		window = append(window, i) // store the index of nums
		if i >= k-1 {
			result = append(result, nums[window[0]]) // the left-most is the index of max value in nums
		}
	}
	return result
}
