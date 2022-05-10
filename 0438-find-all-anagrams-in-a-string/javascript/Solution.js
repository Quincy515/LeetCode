/**
 * @param {string} s
 * @param {string} p
 * @return {number[]}
 */
var findAnagrams = function(s, p) {
    let n = s.length
    let m = p.length
    if (m > n) {
        return []
    }
    let needs = new Array(26).fill(0)
    for (let i = 0; i < m; ++i) {
        needs[charCodeSubA(p[i])]++
    }
    let matched = new Array(26).fill(0)
    let startp = 0
    let endp = 0
    let result = []
    while (endp < m) {
        matched[charCodeSubA(s[endp])]++
        endp++
    }
    if (same(needs, matched)) {
        result.push(startp)
    }
    while (endp < n && startp < n) {
        matched[charCodeSubA(s[startp])]--
        matched[charCodeSubA(s[endp])]++
        startp++
        endp++
        if (same(needs, matched)) {
            result.push(startp)
        }
    }
    return result
};
var same = (needs, matched) => {
    for (let i = 0; i < needs.length; ++i) {
        if (needs[i] !== matched[i]) {
            return false
        }
    }
    return true
}
var charCodeSubA = a => a.charCodeAt(0) - 'a'.charCodeAt(0)
