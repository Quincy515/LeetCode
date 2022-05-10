/**
 * @param {number[]} a
 * @param {number[]} b
 * @return {number}
 */
var smallestDifference = function(a, b) {
    let sortFn = (i, j) => i - j
    a.sort(sortFn)
    b.sort(sortFn)
    let n = a.length
    let m = b.length
    let minRet = Number.MAX_SAFE_INTEGER
    let i = 0
    let j = 0
    while (i < n && j < m) {
        if (a[i] >= b[j]) {
            minRet = Math.min(minRet, a[i] - b[j])
            j++
        } else {
            minRet = Math.min(minRet, b[j] - a[i])
            i++
        }
    }
    return minRet
};