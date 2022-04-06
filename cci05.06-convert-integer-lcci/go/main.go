package main

func convertInteger(A int, B int) int {
	C := A ^ B
	count := 0
	for i := 0; i < 32; i++ {
		if (C & 1) == 1 {
			count++
		}
		C >>= 1
	}
	return count
}
