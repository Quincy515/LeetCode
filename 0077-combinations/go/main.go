package main

var result [][]int

func combine(n int, k int) [][]int {
	result = make([][]int, 0)
	backtrack(n, k, 1, []int{})
	return result
}
func backtrack(n, k, step int, path []int) {
	if len(path) == k {
		snapshot := make([]int, len(path))
		copy(snapshot, path)
		result = append(result, snapshot)
		return
	}
	if step == n+1 {
		return
	}
	backtrack(n, k, step+1, path)
	path = append(path, step)
	backtrack(n, k, step+1, path)
	path = path[:len(path)-1]
}
