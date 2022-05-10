/**
 * @param {string} word1
 * @param {string} word2
 * @return {number}
 */
var minDistance = function(word1, word2) {
    let n = word1.length
    let m = word2.length
    if (n == 0) return m
    if (m == 0) return n
    let dp = new Array(n+1).fill().map(() => new Array(m+1).fill(0))
    for (let j = 0; j <= m; j++) {
        dp[0][j] = j
    }
    for (let i = 0; i <= n; i++) {
        dp[i][0] = i
    }
    for (let i = 1; i <= n; i++) {
        for (let j = 1; j <= m; j++) {
            if (word1[i-1] == word2[j-1]) {
                dp[i][j] = Math.min(dp[i-1][j]+1, dp[i][j-1]+1, dp[i-1][j-1])
            } else {
                dp[i][j] = Math.min(dp[i-1][j]+1, dp[i][j-1]+1, dp[i-1][j-1]+1)
            }
        }
    }
    return dp[n][m]
};