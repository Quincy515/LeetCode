/**
 * @param {number[]} nums
 * @return {boolean}
 */
var canJump = function (nums) {
    let max = 0;
    let n = nums.length;
    for (let i = 0; i < n; i++) {
        if (max >= n - 1) return true;
        if (i > max) return false;
        max = Math.max(max, i + nums[i]);
    }
    return false;
};