package main

func singleNumbers(nums []int) []int {
	xorRexult := 0
	n := len(nums)
	for i := 0; i < n; i++ {
		xorRexult ^= nums[i]
	}
	tag := 1
	for (xorRexult & tag) == 0 {
		tag = tag << 1
	}
	a := 0
	b := 0
	for i := 0; i < n; i++ {
		if (nums[i] & tag) == 0 {
			a ^= nums[i]
		} else {
			b ^= nums[i]
		}
	}
	return []int{a, b}
}
