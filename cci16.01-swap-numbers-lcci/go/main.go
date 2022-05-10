package main

func swapNumbers(numbers []int) []int {
	if numbers[0] == numbers[1] {
		return numbers
	}
	numbers[0] ^= numbers[1]
	numbers[1] ^= numbers[0]
	numbers[0] ^= numbers[1]
	return numbers
}
