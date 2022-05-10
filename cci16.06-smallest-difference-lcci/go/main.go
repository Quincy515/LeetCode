package main

import (
	"math"
	"sort"
)

func smallestDifference(a []int, b []int) int {
	sort.Ints(a)
	sort.Ints(b)
	n := len(a)
	m := len(b)
	var minRet int64 = math.MaxInt64
	i := 0
	j := 0
	for i < n && j < m {
		if a[i] >= b[j] {
			minRet = int64(math.Min(float64(minRet), float64(a[i]-b[j])))
			j++
		} else {
			minRet = int64(math.Min(float64(minRet), float64(b[j]-a[i])))
			i++
		}
	}
	return int(minRet)
}
