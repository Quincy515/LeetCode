/**
 * @param {number} x
 * @param {number} y
 * @return {number}
 */
var hammingDistance = function(x, y) {
    let r = x ^ y
    let mask = 1
    let count = 0
    for (let i = 0; i < 31; i++) {
        if ((r & mask) != 0) {
            count++
        }
        mask *= 2
    }
    return count
};