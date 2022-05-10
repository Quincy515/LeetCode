/**
 * @param {number[]} prices
 * @return {number}
 */
var maxProfit = function(prices) {
    let n = prices.length
    if (n == 0) {
        return 0
    }
    let dp = Array(n).fill().map(() => Array(4).fill(0))
    dp [0][0] = -prices[0]
    dp[0][1] = 0
    dp[0][2] = 0
    dp[0][3] = 0
    for (let i = 1; i < n; ++i) {
        dp[i][0] = Math.max(dp[i-1][0], dp[i-1][2]-prices[i], dp[i-1][3]-prices[i])
        dp[i][1] = dp[i-1][0]+prices[i]
        dp[i][2] = dp[i-1][1]
        dp[i][3] = Math.max(dp[i-1][2], dp[i-1][3])
    }
    return Math.max(...dp[n-1])
};