package main

func canConstruct(s string, k int) bool {
	if len(s) < k {
		return false
	}
	if len(s) == k {
		return true
	}
	// 1. 统计每个字符出现的频次
	m := make(map[rune]int)
	for _, c := range s {
		m[c]++
	}
	// 2. 统计出现奇数次字符的个数
	var oddNumCnt int
	for _, v := range m {
		if v%2 == 1 {
			oddNumCnt++
		}
	}
	// 3. 判断有多个在一个回文串中是无用的，因为回文串中只能放一个落单的字符，就是把它放在正中心
	if k <= (oddNumCnt - 1) {
		return false
	}
	return true
}