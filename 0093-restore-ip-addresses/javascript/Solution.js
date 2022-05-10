/**
 * @param {string} s
 * @return {string[]}
 */
var restoreIpAddresses = function(s) {
    let result = []
    let path = []
    let backtrack = (k, step) => {
        if (k == s.length && step == 4) {
            result.push(path.join("."))
            return
        }
        if (step > 4) {
            return
        }
        if (k == s.length) {
            return
        }
        let val = 0
        if (k < s.length) {
            val = val * 10 + (s[k].charCodeAt(0)-'0'.charCodeAt(0))
            path.push(val)
            backtrack(k + 1, step + 1)
            path.pop()
        }
        if (s[k] == '0') {
            return
        }
        if (k+1 < s.length) {
            val = val * 10 + (s[k+1].charCodeAt(0) - '0'.charCodeAt(0))
            path.push(val)
            backtrack(k+2, step+1)
            path.pop()
        }
        if (k+2 < s.length) {
            val = val * 10 + (s[k+2].charCodeAt(0) - '0'.charCodeAt(0))
            if (val <= 255) {
                path.push(val)
                backtrack(k+3, step+1)
                path.pop()
            }
        }
    }
    backtrack(0, 0)
    return result
};