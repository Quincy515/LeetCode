package main

func plusOne(digits []int) []int {
	for i := len(digits) - 1; i >= 0; i-- {
		if digits[i] < 9 {
			// 数字非9，直接加一返回
			digits[i]++
			return digits
		}
		// 数字是9，将其置为0
		digits[i] = 0
	}
	// 未提前返回，则i=-1,表示全部数字是9，重置数组，数组长度因进位而加一，除第一个元素为1外，其余元素皆为0
	return append([]int{1}, digits...)
}
