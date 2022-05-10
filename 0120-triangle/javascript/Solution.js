/**
 * @param {number[][]} triangle
 * @return {number}
 */
var minimumTotal = function(triangle) {
    let n = triangle.length
    let dp = Array(n).fill().map(() => Array(n).fill(0))
    dp[0][0] = triangle[0][0]
    for (let i = 1; i < n; ++i) {
        dp[i][0] = dp[i-1][0] + triangle[i][0]
        for (let j = 1; j < i; ++j) {
            dp[i][j] = Math.min(dp[i-1][j], dp[i-1][j-1]) + triangle[i][j]
        }
        dp[i][i] = dp[i-1][i-1] + triangle[i][i]
    }
    let res = Number.MAX_SAFE_INTEGER
    for (let j = 0; j < n; ++j) {
        if (dp[n-1][j] < res) {
            res = dp[n-1][j]
        }
    }
    return res
};