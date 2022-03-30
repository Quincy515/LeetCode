/**
 * @param {number} n
 * @return {number}
 */
var cuttingRope = function(n) {
    if (n == 1) return 1
    if (n == 2) return 1
    if (n == 3) return 2
    let dp = new Array(n+1).fill(0)
    dp[0] = 1
    for (let i = 1; i <= n; ++i) {
        for (let j = 1; j <= i; j++) {
            if (dp[i] < j*dp[i-j]) {
                dp[i] = j*dp[i-j]
            }
        }
    }
    return dp[n]
};