/**
 * @param {number} A
 * @param {number} B
 * @return {number}
 */
var convertInteger = function(A, B) {
    let C = A ^ B
    let count = 0
    for (let i = 0; i < 32; ++i) {
        if ((C&1) == 1) {
            count++
        }
        C >>= 1
    }
    return count
};
