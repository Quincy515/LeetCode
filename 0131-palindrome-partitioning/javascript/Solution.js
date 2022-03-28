/**
 * @param {string} s
 * @return {string[][]}
 */
var partition = function(s) {
    let result = []
    let path = []
    let backtrack = k => {
        if (k == s.length) {
            result.push([...path])
            return
        }
        for (let end = k; end < s.length; ++end) {
            if (ispalindrome(s, k, end)) {
                path.push(s.substring(k, end+1))
                backtrack(end+1)
                path.pop()
            }
        }
    }
    backtrack(0)
    return result
};
var ispalindrome = (s, p, r) => {
    let i = p
    let j = r
    while (i <= j) {
        if (s[i] != s[j]) {
            return false
        }
        i++
        j--
    }
    return true
}