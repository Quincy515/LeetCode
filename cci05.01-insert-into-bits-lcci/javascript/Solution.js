/**
 * @param {number} N
 * @param {number} M
 * @param {number} i
 * @param {number} j
 * @return {number}
 */
var insertBits = function(N, M, i, j) {
    let nbits = new Array(32).fill(0)
    let mbits = new Array(32).fill(0)
    let mask = 1
    for (let k = 0; k < 32; ++k) {
        if ((N&mask) != 0) {
            nbits[k] = 1
        }
        mask <<= 1
    }
    mask = 1
    for (let k = 0; k < 32; ++k) {
        if ((M&mask) != 0) {
            mbits[k] = 1
        }
        mask <<= 1
    }
    for (let k = i; k <= j; ++k) {
        nbits[k] = mbits[k-i]
    }
    mask = 1
    let ret = 0
    for (let k = 0; k < 32; ++k) {
        ret += (nbits[k] * mask)
        mask <<= 1
    }
    return ret
};
