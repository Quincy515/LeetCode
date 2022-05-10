/**
 * @param {number} num
 * @return {number}
 */
var exchangeBits = function(num) {
    let ret = 0
    for (let i = 0; i <= 30; i+=2) {
        let a1 = (num & (1 << i))
        let b1 = (num & (1 << (i+1)))
        if (a1 != 0) {
            ret |= (1 << (i+1))
        }
        if (b1 != 0) {
            ret |= (1 << i)
        }
    }
    return ret
};
