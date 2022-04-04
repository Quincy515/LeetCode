/**
 * @param {string[]} words
 * @param {string} word1
 * @param {string} word2
 * @return {number}
 */
var findClosest = function(words, word1, word2) {
    let w1ps = []
    let w2ps = []
    for (let i = 0; i < words.length; ++i) {
        let word = words[i]
        if (word === word1) {
            w1ps.push(i)
        } else if (word === word2) {
            w2ps.push(i)
        }
    }
    let p1 = 0
    let p2 = 0
    let minRet = Number.MAX_SAFE_INTEGER
    while (p1 < w1ps.length && p2 < w2ps.length) {
        let pos1 = w1ps[p1]
        let pos2 = w2ps[p2]
        if (pos1 > pos2) {
            if (minRet > pos1-pos2) {
                minRet = pos1-pos2
            }
            p2++
        } else {
            if (minRet > pos2-pos1) {
                minRet = pos2-pos1
            }
            p1++
        }
    }
    return minRet
};