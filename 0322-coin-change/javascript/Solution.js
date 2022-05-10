/**
 * @param {number[]} coins
 * @param {number} amount
 * @return {number}
 */
var coinChange = function(coins, amount) {
    let n = coins.length
    let dp = Array(n).fill().map(() => Array(amount+1).fill(Number.MAX_SAFE_INTEGER))
    for (let c = 0; c <= Math.floor(amount/coins[0]); ++c) {
        dp[0][c*coins[0]] = c
    }
    for (let i = 1; i < n; ++i) {
        for (let j = 0; j <= amount; ++j) {
            let k = Math.floor(j/coins[i])
            for (let c = 0; c <= k; ++c) {
                if (dp[i-1][j-c*coins[i]] != Number.MAX_SAFE_INTEGER &&
                    dp[i-1][j-c*coins[i]] + c < dp[i][j]) {
                    dp[i][j] = dp[i-1][j-c*coins[i]] + c
                }
            }
        }
    }
    if (dp[n-1][amount] == Number.MAX_SAFE_INTEGER) {
        return -1
    }
    return dp[n-1][amount]
};

/**
 * @param {number[]} coins
 * @param {number} amount
 * @return {number}
 */
var coinChange = function(coins, amount) {
    let k = coins.length
    let dp = Array(amount+1).fill(Number.MAX_SAFE_INTEGER)
    dp[0] = 0
    for (let i = 1; i <= amount; ++i) {
        for (let j = 0; j < k; ++j) {
            if (i-coins[j] >= 0 &&
                dp[i-coins[j]] != Number.MAX_SAFE_INTEGER &&
                dp[i-coins[j]] + 1 < dp[i]) {
                dp[i] = dp[i-coins[j]] + 1
            }
        }
    }
    if (dp[amount] == Number.MAX_SAFE_INTEGER) {
        return -1
    }
    return dp[amount]
};