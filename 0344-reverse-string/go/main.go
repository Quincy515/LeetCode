package main

func reverseString(s []byte) {
	n := len(s)
	i := 0
	j := n - 1
	for i <= j {
		s[i], s[j] = s[j], s[i]
		i++
		j--
	}
}
