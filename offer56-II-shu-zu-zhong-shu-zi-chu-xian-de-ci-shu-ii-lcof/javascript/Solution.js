/**
 * @param {number[]} nums
 * @return {number}
 */
var singleNumber = function(nums) {
    let n = nums.length
    let bits = new Array(32).fill(0)
    let mask = 1
    for (let i = 0; i < 32; ++i) {
        for (let j = 0; j < n; ++j) {
            if ((nums[j] & mask) != 0) {
                bits[i] = (bits[i] + 1) % 3
            }
        }
        mask <<= 1
    }
    let result = 0
    mask = 1
    for (let i = 0; i < 32; ++i) {
        if (bits[i] == 1) {
            result += mask
        }
        mask <<= 1
    }
    return result
};
