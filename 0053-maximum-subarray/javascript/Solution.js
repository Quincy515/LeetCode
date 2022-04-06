/**
 滑动窗⼝
 * @param {number[]} nums
 * @return {number}
 */
var maxSubArray = function(nums) {
    let n = nums.length
    let maxSum = Number.MIN_SAFE_INTEGER
    let sum = 0
    for (let i = 0; i < n; ++i) {
        if (sum < 0) {
            sum = 0
        }
        sum += nums[i]
        if (sum > maxSum) {
            maxSum = sum
        }
    }
    return maxSum
};

/**
 前缀后缀统计
 * @param {number[]} nums
 * @return {number}
 */
var maxSubArray = function(nums) {
    if (nums.length == 1) {
        return nums[0]
    }
    let sum = new Array(nums.length)
    let max = new Array(nums.length)
    let cursum = 0
    for (let i = 0; i < nums.length; ++i) {
        cursum += nums[i]
        sum[i] = cursum
    }
    let curmax = Number.MIN_SAFE_INTEGER
    for (let i = sum.length-1; i >= 0; --i) {
        if (curmax < sum[i]) {
            curmax = sum[i]
        }
        max[i] = curmax
    }
    let result = Number.MIN_SAFE_INTEGER
    for (let i = 0; i < nums.length; ++i) {
        if (i == 0 && result < max[0]) {
            result = max[0]
        }
        if (i != 0 && result < max[i]-sum[i-1]) {
            result = max[i] - sum[i-1]
        }
    }
    return result
};