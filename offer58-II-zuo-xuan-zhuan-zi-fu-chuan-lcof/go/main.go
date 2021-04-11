package main

func reverseLeftWords(s string, n int) string {
	// 字符串长度赋值给m，n对字符串长度取模，定义游标i
	m, k, i := len(s), n%len(s), 0
	// 定义辅助数组
	t := make([]byte, m)
	// 先把原字符串中后面m-k个字符拷贝到临时字符数组
	for j := k; j < m; j++ {
		t[i] = s[j]
		i++
	}
	// 再把原字符串前面的k个字符拷贝到临时字符数组
	for j := 0; j < k; j++ {
		t[i] = s[j]
		i++
	}
	return string(t)
}
