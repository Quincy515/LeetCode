/**
 * @param {number} n
 * @param {number} k
 * @return {number[][]}
 */
var combine = function(n, k) {
    let result = []
    let path = []
    let backtrack = step => {
        if (path.length == k) {
            result.push([...path])
            return
        }
        if (step == n+1) {
            return
        }
        backtrack(step+1)
        path.push(step)
        backtrack(step+1)
        path.pop()
    }
    backtrack(1)
    return result
};