/**
 * @param {number[]} nums
 * @return {number[][]}
 */
var permute = function(nums) {
    let result = []
    let path = []
    let backtrack = k => {
        if (k == nums.length) {
            result.push([...path])
            return
        }
        for (let i = 0; i < nums.length; ++i) {
            if (path.includes(nums[i])) {
                continue
            }
            path.push(nums[i])
            backtrack(k+1)
            path.pop()
        }
    }
    backtrack(0)
    return result
};