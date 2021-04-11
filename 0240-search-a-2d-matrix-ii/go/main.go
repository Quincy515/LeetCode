package main

// Time: O(m+n), Space: O(1), m和n分别是二维数组的行数和列数
func searchMatrix(matrix [][]int, target int) bool {
	// 检查所有边界情况
	if matrix == nil || len(matrix) == 0 ||
		matrix[0] == nil || len(matrix[0]) == 0 {
		return false // 如果出现直接返回[-1，-1]
	}
	// 然后计算出行数m和列数n
	m, n := len(matrix), len(matrix[0])
	// 初始化游标i为0，j为n-1，表示指向二维数组右上角的数字
	i, j := 0, n-1
	// 当行坐标小于m列坐标大于等于0时
	for i < m && j >= 0 {
		// 如果目标值小于右上角的数字
		if target < matrix[i][j] {
			j-- // 列下标-1，排除这一列数字，向左移动一位
		} else if target > matrix[i][j] { // 如果目标值大于右上角的数字
			i++ // 行坐标+1，排序这一行数字，向下移动一位
		} else { // 如果相等
			return true // 直接返回当前的行列下标
		}
	}
	// 循环结束后如果还没有找到目标值，返回[-1,-1]
	return false
}
