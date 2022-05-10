/**
 * @param {string} s
 * @param {string[]} wordDict
 * @return {boolean}
 */
var wordBreak = function(s, wordDict) {
    let n = s.length
    let dp = new Array(n+1).fill(false)
    dp[0] = true
    for (let i = 1; i <= n; i++) {
        for (let word of wordDict) {
            let len = word.length
            let startp = i - len
            if (startp >= 0 && s.startsWith(word, startp) && dp[i-len]) {
                dp[i] = true
                break
            }
        }
    }
    return dp[n]
};