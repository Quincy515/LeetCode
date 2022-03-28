/**
 * @param {number[]} nums
 * @param {number} target
 * @return {number}
 */
var findTargetSumWays = function(nums, target) {
    if (target > 1000 || target < -1000) {
        return 0
    }
    let n = nums.length
    let offset = 1000
    let w = 2000
    let dp = Array(n).fill().map(() => Array(w+1).fill(0))
    dp[0][offset-nums[0]] += 1
    dp[0][offset+nums[0]] += 1
    for (let i = 1; i < n; ++i) {
        for (let j = 0; j <= w; ++j) {
            if (j-nums[i] >= 0 && j-nums[i] <= w) {
                dp[i][j] = dp[i-1][j-nums[i]]
            }
            if (j+nums[i] >= 0 && j+nums[i] <= w) {
                dp[i][j] += dp[i-1][j+nums[i]]
            }
        }
    }
    return dp[n-1][target+1000]
};