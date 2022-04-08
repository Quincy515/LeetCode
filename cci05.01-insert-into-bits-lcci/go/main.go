package main

func insertBits(N int, M int, i int, j int) int {
	nbits := make([]int, 32)
	mbits := make([]int, 32)
	mask := 1
	for k := 0; k < 32; k++ {
		if N&mask != 0 {
			nbits[k] = 1
		}
		mask <<= 1
	}
	mask = 1
	for k := 0; k < 32; k++ {
		if (M & mask) != 0 {
			mbits[k] = 1
		}
		mask <<= 1
	}
	for k := i; k <= j; k++ {
		nbits[k] = mbits[k-i]
	}
	mask = 1
	ret := 0
	for k := 0; k < 32; k++ {
		ret += (nbits[k] * mask)
		mask <<= 1
	}
	return ret
}
