/**
 * @param {string} digits
 * @return {string[]}
 */
var letterCombinations = function(digits) {
    let result = []
    if (digits.length == 0) {
        return result
    }
    let mappings = new Array(10)
    mappings[2] = "abc"
    mappings[3] = "def"
    mappings[4] = "ghi"
    mappings[5] = "jkl"
    mappings[6] = "mno"
    mappings[7] = "pqrs"
    mappings[8] = "tuv"
    mappings[9] = "wxyz"
    let path = new Array(digits.length)
    let backtrack = k => {
        if (k == digits.length) {
            result.push(path.join(''))
            return
        }

        let mapping = mappings[digits.charAt(k).charCodeAt(0) - '0'.charCodeAt(0)]
        for (let i of mapping) {
            path[k] = i
            backtrack(k+1)
        }
    }
    backtrack(0)
    return result
};