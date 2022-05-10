/**
 * @param {number[]} nums
 * @return {number[]}
 */
var singleNumbers = function(nums) {
    let xorResult = 0
    let n = nums.length
    for (let i = 0; i < n; ++i) {
        xorResult ^= nums[i]
    }
    let tag = 1
    while ((xorResult & tag) == 0) {
        tag = tag << 1
    }
    let a = 0
    let b = 0
    for (let i = 0; i < n; ++i) {
        if ((nums[i] & tag) == 0) {
            a ^= nums[i]
        } else {
            b ^= nums[i]
        }
    }
    return [a, b]
};
