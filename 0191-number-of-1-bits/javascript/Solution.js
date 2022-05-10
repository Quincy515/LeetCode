/**
 * @param {number} n - a positive integer
 * @return {number}
 */
var hammingWeight = function(n) {
    let oneCount = 0
    let mask = 1
    for (let i = 0; i < 32; i++) {
        if ((n & mask) != 0) {
            oneCount++
        }
        mask <<= 1
    }
    return oneCount
};