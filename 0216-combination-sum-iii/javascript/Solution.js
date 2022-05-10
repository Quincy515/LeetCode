/**
 * @param {number} k
 * @param {number} n
 * @return {number[][]}
 */
var combinationSum3 = function(k, n) {
    let result = []
    let path = []
    let backtrack = (step, sum) => {
        if (sum == n && path.length == k) {
            result.push([...path])
            return
        }
        if (sum > n || path.length > k || step > 9) {
            return
        }
        backtrack(step + 1, sum)
        path.push(step)
        backtrack(step + 1, sum + step)
        path.pop()
    }
    backtrack(1, 0)
    return result
};