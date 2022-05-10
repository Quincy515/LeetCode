/**
 * @param {number[]} nums
 * @return {number[][]}
 */
var subsets = function(nums) {
    let result = []
    let path = []
    let backtrack = k => {
        if (k == nums.length) {
            result.push([...path])
            return
        }
        backtrack(k+1)
        path.push(nums[k])
        backtrack(k+1)
        path.pop()
    }
    backtrack(0)
    return result
};