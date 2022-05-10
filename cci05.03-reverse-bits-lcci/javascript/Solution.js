/**
 * @param {number} num
 * @return {number}
 */
var reverseBits = function(num) {
    if (num == 0) {
        return 1
    }
    let nums = new Array(32)
    for (let i = 0; i < 32; ++i) {
        nums[i] = (num&1)
        num >>= 1
    }
    let leftOneCounts = new Array(32)
    let count = 0
    for (let i = 0; i < 32; ++i) {
        leftOneCounts[i] = count
        if (nums[i] == 1) {
            count++
        } else {
            count = 0
        }
    }
    let rightOneCounts = new Array(32)
    count = 0
    for (let i = 31; i >= 0; --i) {
        rightOneCounts[i] = count
        if (nums[i] == 1) {
            count++
        } else {
            count = 0
        }
    }
    let ret = 0
    for (let i = 0; i < 32; ++i) {
        if (ret < leftOneCounts[i] + rightOneCounts[i] + 1) [
            ret = leftOneCounts[i] + rightOneCounts[i] + 1
        ]
    }
    return ret
};