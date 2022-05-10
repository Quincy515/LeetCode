/**
 * @param {number[]} nums
 * @return {boolean}
 */
var canPartition = function(nums) {
    let n = nums.length
    let sum = 0
    sum = nums.reduce((sum, currentValue) => sum += currentValue, 0)
    if (sum % 2 == 1) {
        return false
    }
    sum = Math.floor(sum / 2)
    let dp = Array(n).fill().map(() => Array(sum+1).fill(false))
    dp[0][0] = true
    if (nums[0] <= sum) {
        dp[0][nums[0]] = true
    }
    for (let i = 1; i < n; ++i) {
        for (let j = 0; j <= sum; ++j) {
            if (j-nums[i] >= 0) {
                dp[i][j] = dp[i-1][j] || dp[i-1][j-nums[i]]
            } else {
                dp[i][j] = dp[i-1][j]
            }
        }
    }
    return dp[n-1][sum]
};