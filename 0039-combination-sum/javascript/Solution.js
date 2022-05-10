/**
 * @param {number[]} candidates
 * @param {number} target
 * @return {number[][]}
 */
var combinationSum = function(candidates, target) {
    let result = []
    let path = []
    let backtrack = (k, left) => {
        if (left == 0) {
            result.push([...path])
            return
        }
        if (k == candidates.length) {
            return
        }
        for (let i = 0; i <= Math.floor(left/candidates[k]); ++i) {
            for (let j = 0; j < i; ++j) {
                path.push(candidates[k])
            }
            backtrack(k+1, left-i*candidates[k])
            for (let j = 0; j < i; ++j) {
                path.pop()
            }
        }
    }
    backtrack(0, target)
    return result
};