/**
 * @param {number[]} candidates
 * @param {number} target
 * @return {number[][]}
 */
var combinationSum2 = function(candidates, target) {
    let result = []
    let hashTable = new Map()
    for (const candidate of candidates) {
        if (!hashTable.has(candidate)) {
            hashTable.set(candidate, 1)
        } else {
            hashTable.set(candidate, hashTable.get(candidate) + 1)
        }
    }
    let nums = []
    let counts = []
    for (const candidate of candidates) {
        if (hashTable.has(candidate)) {
            nums.push(candidate)
            counts.push(hashTable.get(candidate))
            hashTable.delete(candidate)
        }
    }
    let path = []
    let backtrack = (k, left) => {
        if (left == 0) {
            result.push([...path])
            return
        }
        if (left < 0 || k == nums.length) {
            return
        }
        for (let count = 0; count <= counts[k]; ++count) {
            for (let i = 0; i < count; ++i) {
                path.push(nums[k])
            }
            backtrack(k+1, left-count*nums[k])
            for (let i = 0; i < count; ++i) {
                path.pop()
            }
        }
    }
    backtrack(0, target)
    return result
};