/**
 * @param {number[]} prices
 * @return {number}
 */
var maxProfit = function(prices) {
    let n = prices.length
    let max = new Array(n)
    let curMax = 0
    for (let i = n-1; i >= 0; --i) {
        max[i] = curMax
        if (prices[i] > curMax) [
            curMax = prices[i]
        ]
    }
    let result = 0
    for (let i = 0; i < n; ++i) {
        if (result < max[i]-prices[i]) {
            result = max[i]-prices[i]
        }
    }
    return result
};