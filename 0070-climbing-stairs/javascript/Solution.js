/**
 * @param {number} n
 * @return {number}
 */
var climbStairs = function(n) {
    if (n <= 2) {
        return n
    }
    let dp = new Array(n+1)
    dp[1] = 1
    dp[2] = 2
    for (let i = 3; i <= n; ++i) {
        dp[i] = dp[i-1] + dp[i-2]
    }
    return dp[n]
};

/**
 * @param {number} n
 * @return {number}
 */
var climbStairs = function(n) {
    let dp = Array(n+1).fill(0)
    dp[0] = 1
    for (let i = 1; i <= n; ++i) {
        if (i-1 >= 0) {
            dp[i] += dp[i-1]
        }
        if (i-2 >= 0) {
            dp[i] += dp[i-2]
        }
    }
    return dp[n]
};