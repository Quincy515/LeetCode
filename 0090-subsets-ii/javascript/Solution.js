/**
 * @param {number[]} nums
 * @return {number[][]}
 */
var subsetsWithDup = function(nums) {
    let result = []
    let hm = new Map()
    for (let i = 0; i < nums.length; ++i) {
        let count = 1
        if (hm.has(nums[i])) {
            count += hm.get(nums[i])
        }
        hm.set(nums[i], count)
    }
    let n = hm.size
    let uniqueNums = new Array(n)
    let counts = new Array(n)
    let k = 0
    for (let i = 0; i < nums.length; ++i) {
        if (hm.has(nums[i])) {
            uniqueNums[k] = nums[i]
            counts[k] = hm.get(nums[i])
            k++
            hm.delete(nums[i])
        }
    }
    let path = []
    let backtrack = k => {
        if (k == uniqueNums.length) {
            result.push([...path])
            return
        }
        for (let count = 0; count <= counts[k]; ++count) {
            for (let i = 0; i < count; ++i) {
                path.push(uniqueNums[k])
            }
            backtrack(k+1)
            for (let i = 0; i < count; ++i) {
                path.pop()
            }
        }
    }
    backtrack(0)
    return result
};