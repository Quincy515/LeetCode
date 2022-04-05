/**
 * @param {string} s
 * @param {string} t
 * @return {string}
 */
var minWindow = function(s, t) {
    let minWSize = Number.MAX_SAFE_INTEGER
    let minWSStart = -1
    let minWEnd = -1
    let tmap = new Map()
    let wmap = new Map()
    for (let i = 0; i < t.length; ++i) {
        let count = 1
        if (tmap.has(t[i])) {
            count += tmap.get(t[i])
        }
        tmap.set(t[i], count)
    }
    let n = s.length
    let l = 0
    let r = -1
    while (l < n && r < n) {
        while (!match(wmap, tmap)) {
            r++
            if (r > n-1) {
                break
            }
            let c = s[r]
            if (tmap.has(c)) {
                let count = 1
                if (wmap.has(c)) {
                    count += wmap.get(c)
                }
                wmap.set(c, count)
            }
        }
        if (match(wmap, tmap)) {
            if (minWSize > r-l+1) {
                minWSize = r-l+1
                minWSStart = l
                minWEnd = r
            }
            let c = s[l]
            if (tmap.has(c)) {
                let count = wmap.get(c)
                if (count - 1 == 0) {
                    wmap.delete(c)
                } else {
                    wmap.set(c, count-1)
                }
            }
            l++
        }
    }
    if (minWSStart == -1) {
        return ""
    }
    return s.substring(minWSStart, minWEnd+1)

};
var match = (wmap, tmap) => {
    for (let [key, value] of tmap) {
        if (!wmap.has(key)) {
            return false
        }
        if (wmap.get(key) < value) {
            return false
        }
    }
    return true
}