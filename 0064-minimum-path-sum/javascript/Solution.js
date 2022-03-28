/**
 * @param {number[][]} grid
 * @return {number}
 */
var minPathSum = function(grid) {
    let m = grid.length
    let n = grid[0].length
    let dp = Array(m).fill().map(() => Array(n).fill(0))
    let len = 0
    for (let i = 0; i < m; ++i) {
        len += grid[i][0]
        dp[i][0] = len
    }
    len = 0
    for (let j = 0; j < n; ++j) {
        len += grid[0][j]
        dp[0][j] = len
    }
    for (let i = 1; i < m; ++i) {
        for (let j = 1; j < n; ++j) {
            dp[i][j] = Math.min(dp[i-1][j], dp[i][j-1]) + grid[i][j]
        }
    }
    return dp[m-1][n-1]
};