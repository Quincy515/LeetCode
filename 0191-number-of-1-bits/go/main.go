package main

func hammingWeight(num uint32) int {
	oneCount := 0
	var mask uint32 = 1
	for i := 0; i < 32; i++ {
		if num&mask != 0 {
			oneCount++
		}
		mask <<= 1
	}
	return oneCount
}
func hammingWeight(num uint32) int {
	oneCount := 0
	for num != 0 {
		if num&1 == 1 {
			oneCount++
		}
		num >>= 1
	}
	return oneCount
}
