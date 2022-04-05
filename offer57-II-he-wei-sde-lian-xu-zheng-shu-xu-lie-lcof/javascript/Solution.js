/**
 * @param {number} target
 * @return {number[][]}
 */
var findContinuousSequence = function(target) {
    let result = []
    let p = 1
    let q = 2
    let sum = 3
    while (p < q) {
        if (sum === target) {
            let arr = new Array(q-p+1)
            for (let i = p; i <= q; ++i) {
                arr[i-p] = i
            }
            result.push(arr)
            sum -= p
            p++
            q++
            sum += q
        } else if (sum > target) {
            sum -= p
            p++
        } else {
            q++
            sum += q
        }
    }
    return result
};