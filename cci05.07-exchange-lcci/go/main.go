package main

func exchangeBits(num int) int {
	ret := 0
	for i := 0; i <= 30; i += 2 {
		a1 := (num & (1 << i))
		b1 := (num & (1 << (i + 1)))
		if a1 != 0 {
			ret |= (1 << (i + 1))
		}
		if b1 != 0 {
			ret |= (1 << i)
		}
	}
	return ret
}
