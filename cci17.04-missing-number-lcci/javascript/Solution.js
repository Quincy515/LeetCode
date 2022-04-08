/**
 * @param {number[]} nums
 * @return {number}
 */
var missingNumber = function(nums) {
    let n = nums.length
    let ret = 0
    for (let i = 0; i <= n; ++i) {
        ret ^= i
    }
    for (let i = 0; i < n; ++i) {
        ret ^= nums[i]
    }
    return ret
};
