package main

func findContinuousSequence(target int) [][]int {
	result := make([][]int, 0)
	p := 1
	q := 2
	sum := 3
	for p < q {
		if sum == target {
			arr := make([]int, q-p+1)
			for i := p; i <= q; i++ {
				arr[i-p] = i
			}
			result = append(result, arr)
			sum -= p
			p++
			q++
			sum += q
		} else if sum > target {
			sum -= p
			p++
		} else {
			q++
			sum += q
		}
	}
	return result
}
