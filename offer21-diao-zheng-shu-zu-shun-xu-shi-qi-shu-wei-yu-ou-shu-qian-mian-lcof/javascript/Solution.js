/**
 * @param {number[]} nums
 * @return {number[]}
 */
var exchange = function(nums) {
    let i = 0
    let j = nums.length - 1
    while (i < j) {
        if (nums[i] % 2 === 1) {
            i++
            continue
        }
        if (nums[j] % 2 === 0) {
            j--
            continue
        }
        [nums[i], nums[j]] = [nums[j], nums[i]]
        i++
        j--
    }
    return nums
};