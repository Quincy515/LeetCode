/**
 * @param {number[]} nums
 * @return {number[][]}
 */
var permuteUnique = function(nums) {
    let result = []
    let hm = new Map()
    for (let num of nums) {
        let count = 1
        if (hm.has(num)) {
            count += hm.get(num)
        }
        hm.set(num, count)
    }
    let n = hm.size
    let uniqueNums = new Array(n)
    let counts = new Array(n)
    let k = 0
    for (let num of nums) {
        if (hm.has(num)){
            uniqueNums[k] = num
            counts[k] = hm.get(num)
            k++
            hm.delete(num)
        }
    }
    let path = []
    let backtrack = k => {
        if (k == nums.length) {
            result.push([...path])
            return
        }
        for (let i = 0; i < uniqueNums.length; ++i) {
            if (counts[i] == 0) {
                continue
            }
            path.push(uniqueNums[i])
            counts[i] -= 1
            backtrack(k+1)
            path.pop()
            counts[i] += 1
        }
    }
    backtrack(0)
    return result
};