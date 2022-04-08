package main

func singleNumber(nums []int) int {
	n := len(nums)
	bits := make([]int, 32)
	mask := 1
	for i := 0; i < 32; i++ {
		for j := 0; j < n; j++ {
			if (nums[j] & mask) != 0 {
				bits[i] = (bits[i] + 1) % 3
			}
		}
		mask <<= 1
	}
	result := 0
	mask = 1
	for i := 0; i < 32; i++ {
		if bits[i] == 1 {
			result += mask
		}
		mask <<= 1
	}
	return result
}