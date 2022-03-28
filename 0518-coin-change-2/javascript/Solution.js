/**
 * @param {number} amount
 * @param {number[]} coins
 * @return {number}
 */
var change = function(amount, coins) {
    let n = coins.length
    let dp = Array(n).fill().map(() => Array(amount+1).fill(0))
    for (let c = 0; c <= Math.floor(amount/coins[0]); ++c) {
        dp[0][c*coins[0]] = 1
    }
    for (let i = 1; i < n; ++i) {
        for (let j = 0; j <= amount; ++j) {
            let k = Math.floor(j/coins[i])
            for (let c = 0; c <= k; ++c) {
                dp[i][j] += dp[i-1][j-c*coins[i]]
            }
        }
    }
    return dp[n-1][amount]
};