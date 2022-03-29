/**
 * @param {number[][]} grid
 * @return {number}
 */
var maxValue = function(grid) {
    let n = grid.length
    let m = grid[0].length
    let dp = Array(n).fill().map(() => Array(m).fill(0))
    let sum = 0
    for (let j = 0; j < m; ++j) {
        sum += grid[0][j]
        dp[0][j] = sum
    }
    sum = 0
    for (let i = 0; i < n; ++i) {
        sum += grid[i][0]
        dp[i][0] = sum
    }
    for (let i = 1; i < n; ++i) {
        for (let j = 1; j < m; ++j) {
            dp[i][j] = Math.max(dp[i-1][j], dp[i][j-1]) + grid[i][j]
        }
    }
    return dp[n-1][m-1]
};