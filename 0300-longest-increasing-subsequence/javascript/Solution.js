/**
 * @param {number[]} nums
 * @return {number}
 */
var lengthOfLIS = function(nums) {
    let n = nums.length
    let dp = new Array(n).fill(0)
    dp[0] = 1
    for (let i = 1; i < n; ++i) {
        dp[i] = 1
        for (let j = 0; j < i; ++j) {
            if (nums[i] > nums[j]) {
                dp[i] = Math.max(dp[i], dp[j] + 1)
            }
        }
    }
    return Math.max(...dp)
};
/**
 * @param {number[]} nums
 * @return {number}
 */
var lengthOfLIS = function(nums) {
    let n = nums.length
    let lisToMinV = new Array(n+1).fill(0)
    let k = 0
    let dp = new Array(n).fill(0)
    for (let i = 0; i < n; ++i) {
        let len = bsearch(lisToMinV, k, nums[i])
        if (len == -1) {
            dp[i] = 1
        } else {
            dp[i] = len + 1
        }
        if (dp[i] > k) {
            k = dp[i]
            lisToMinV[dp[i]] = nums[i]
        } else if (lisToMinV[dp[i]] > nums[i]) {
            lisToMinV[dp[i]] = nums[i]
        }
    }
    return Math.max(...dp)
};
var bsearch = (a, k, target) => {
    let low = 1
    let high = k
    while (low <= high) {
        let mid = Math.floor((low+high)/2)
        if (a[mid] < target) {
            if (mid == k || a[mid+1] >= target) {
                return mid
            } else {
                low = mid + 1
            }
        } else {
            high = mid - 1
        }
    }
    return -1 }