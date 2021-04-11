package main

func isPalindrome(x int) bool {
	// 双端队列，头尾各取一个数，相同就丢弃，不同就返回false
	var dequeue []int
	if x < 0 {
		return false
	}
	// 取每一位数字加入到dequeue
	for x != 0 {
		dequeue = append(dequeue, x%10)
		x = x / 10
	}
	for i, j := 0, len(dequeue)-1; i <= j; i, j = i+1, j-1 {
		if dequeue[i] != dequeue[j] {
			return false
		}
	}
	return true
}
