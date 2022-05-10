/**
 * @param {number[]} height
 * @return {number}
 */
var trap = function(height) {
    let n = height.length
    let result = 0
    for (let i = 1; i < n-1; ++i) {
        let lh = 0
        for (let j = 0; j < i; ++j) {
            if (height[j] > lh) {
                lh = height[j]
            }
        }
        let rh = 0
        for (let j = i+1; j < n; ++j) {
            if (height[j] > rh) {
                rh = height[j]
            }
        }
        let carry = Math.min(lh, rh) - height[i]
        if (carry < 0) {
            carry = 0
        }
        result += carry
    }
    return result
};
// 前缀/后缀统计解法
/**
 * @param {number[]} height
 * @return {number}
 */
var trap = function(height) {
    let n = height.length

    let leftMax = new Array(n)
    let max = 0
    for (let i = 0; i < n; ++i) {
        leftMax[i] = Math.max(max, height[i])
        max = leftMax[i]
    }
    let rightMax = new Array(n)
    max = 0
    for (let i = n-1; i >= 0; --i) {
        rightMax[i] = Math.max(max, height[i])
        max = rightMax[i]
    }
    let result = 0
    for (let i = 1; i < n-1; i++) {
        result += Math.min(leftMax[i], rightMax[i]) - height[i]
    }
    return result
};