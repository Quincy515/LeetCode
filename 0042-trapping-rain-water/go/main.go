package main

import "math"

func trap(height []int) int {
	n := len(height)
	result := 0 //遍历每个柱子 n，查找它左边的最高柱子 lh，和有变得最高柱子 rh //柱子上能承载的雨水=min(lh,rh)-h
	for i := 1; i < n-1; i++ {
		lh := 0
		for j := 0; j < i; j++ { // 左侧最高 lh
			if height[j] > lh {
				lh = height[j]
			}
		}
		rh := 0
		for j := i + 1; j < n; j++ { // 右侧最高 rh
			if height[j] > rh {
				rh = height[j]
			}
		}
		carry := int(math.Min(float64(lh), float64(rh))) - height[i]
		if carry < 0 {
			carry = 0
		}
		result += carry
	}
	return result
}

//解法 2
func trap(height []int) int {
	n := len(height) // 前缀 max
	leftMax := make([]int, n)
	max := 0
	for i := 0; i < n; i++ {
		leftMax[i] = int(math.Max(float64(max), float64(height[i])))
		max = leftMax[i]
	} // 后缀 max
	rightMax := make([]int, n)
	max = 0
	for i := n - 1; i >= 0; i-- {
		rightMax[i] = int(math.Max(float64(max), float64(height[i])))
		max = rightMax[i]
	} // 每个柱子上承载的雨水
	result := 0
	for i := 1; i < n-1; i++ {
		result += int(math.Min(float64(leftMax[i]), float64(rightMax[i]))) - height[i]
	}
	return result
}
