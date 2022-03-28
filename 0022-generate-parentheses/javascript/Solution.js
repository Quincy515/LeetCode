/**
 * @param {number} n
 * @return {string[]}
 */
var generateParenthesis = function(n) {
    let result = []
    let path = new Array(2*n)
    let backtrack = (leftUsed, rightUsed, k) => {
        if (k == 2*n) {
            result.push(path.join(""))
            return
        }
        if (leftUsed < n) {
            path[k] = '('
            backtrack(leftUsed+1, rightUsed, k+1)
        }
        if (leftUsed > rightUsed) {
            path[k] = ')'
            backtrack(leftUsed, rightUsed+1, k+1)
        }
    }
    backtrack(0,0,0)
    return result
};