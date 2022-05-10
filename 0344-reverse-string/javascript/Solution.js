/**
 * @param {character[]} s
 * @return {void} Do not return anything, modify s in-place instead.
 */
var reverseString = function(s) {
    const n = s.length
    let i = 0
    let j = n - 1
    while (i <= j) {
        [s[i], s[j]] = [s[j], s[i]]
        i++
        j--
    }
};