/**
 * @param {number} num
 * @return {number}
 */
var translateNum = function(num) {
    if (num <= 9) return 1
    let digitlist = []
    while (num != 0) {
        digitlist.push(num % 10)
        num = Math.floor(num/10)
    }
    let n = digitlist.length
    let digits = new Array(n)
    for (let i = 0; i < n; ++i) {
        digits[i] = digitlist[n-i-1]
    }
    let dp = new Array(n+1).fill(0)
    dp[0] = 1
    for (let i = 1; i <= n; ++i) {
        dp[i] = dp[i-1]
        if (i-2 >= 0 && isValid(digits[i-2], digits[i-1])) {
            dp[i] += dp[i-2]
        }
    }
    return dp[n]
};
var isValid = (a, b) => {
    if (a == 1) return true
    if (a == 2 && b>=0 && b<=5) return true
    return false
}