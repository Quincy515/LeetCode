package main

func wordBreak(s string, wordDict []string) bool {
	n := len(s)
	dp := make([]bool, n+1)
	dp[0] = true
	for i := 1; i <= n; i++ {
		for _, word := range wordDict {
			len := len(word)
			startp := i - len
			if startp >= 0 && startsWith(s, word, startp) && dp[i-len] {
				dp[i] = true
				break
			}
		}
	}
	return dp[n]
}

func startsWith(s, word string, start int) bool {
	return s[start:start+len(word)] == word
}
