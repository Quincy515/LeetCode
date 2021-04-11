package main

// Time: O(m*n), Space: O(1)
func spiralOrder(matrix [][]int) []int {
	// 如果矩阵为空，或长度为0
	if matrix == nil || len(matrix) == 0 ||
		matrix[0] == nil || len(matrix[0]) == 0 {
		return []int{} // 返回空序列
	}
	var result []int //  否则定义结果列表
	// 同时需要定义上下左右四个边界并初始化它们
	top, bottom, left, right := 0, len(matrix)-1, 0, len(matrix[0])-1
	for { // 接着不断循环一下操作
		// 1. 先从左到右遍历上边界，把元素加到结果列表
		for i := left; i <= right; i++ {
			result = append(result, matrix[top][i])
		}
		top++ // 然后top+1
		// 同时判断top是否已经大于bottom
		if top > bottom {
			break // 是的话就退出循环
		}
		// 2. 然后从上向下遍历右边界，把元素加到结果列表
		for i := top; i <= bottom; i++ {
			result = append(result, matrix[i][right])
		}
		right-- // 然后right-1
		// 同时判断right是否已经小于left
		if right < left {
			break // 是的话就退出循环
		}
		// 3. 然后从右向左遍历下边界，把元素加到结果列表
		for i := right; i >= left; i-- {
			result = append(result, matrix[bottom][i])
		}
		bottom-- // 然后bottom-1
		// 同时判断bottom是否已经小于top
		if bottom < top {
			break // 是的话就退出循环
		}
		// 4. 然后从下向上遍历左边界，把元素加到结果列表
		for i := bottom; i >= top; i-- {
			result = append(result, matrix[i][left])
		}
		left++ // 然后left+1
		// 同时判断left是否已经大于right
		if left > right {
			break // 是的话就退出循环
		}
	}
	return result // 退出循环后返回结果列表即可。
}
