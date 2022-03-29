/**
 * @param {number[]} nums
 * @return {number}
 */
var rob = function(nums) {
    let n = nums.length
    if (n == 1) {
        return nums[0]
    }
    if (n == 2) {
        return Math.max(nums[0], nums[1])
    }
    let rob_dp = (p, r) => {
        let dp = Array(n).fill().map(() => Array(2).fill(0))
        dp[p][0] = 0
        dp[p][1] = nums[p]
        for (let i = p+1; i <= r; ++i) {
            dp[i][0] = Math.max(dp[i-1][0], dp[i-1][1])
            dp[i][1] = dp[i-1][0] + nums[i]
        }
        return Math.max(dp[r][0], dp[r][1])
    }
    let max1 = rob_dp(1, n-1)
    let max2 = nums[0] + rob_dp(2, n-2)
    return Math.max(max1, max2)
};