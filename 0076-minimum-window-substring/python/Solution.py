class Solution:
    def minWindow(self, s: str, t: str) -> str:
        minWSize = float("inf")
        minWStart = -1
        minWEnd = -1
        tmap = dict()
        wmap = dict()
        for i in range(len(t)):
            count = 1
            if t[i] in tmap:
                count += tmap[t[i]]
            tmap[t[i]] = count
        n = len(s)
        l = 0
        r = -1
        while l < n and r < n:
            while not self.match(wmap, tmap):
                r += 1
            if r > n - 1:
                break
            c = s[r]
            if c in tmap:
                count = 1
                if c in wmap:
                    count += wmap[c]
                wmap[c] = count
        if self.match(wmap, tmap):
            if minWSize > r - l + 1:
                minWSize = r - l + 1
                minWStart = l
                minWEnd = r
            c = s[l]
            if c in tmap:
                count = wmap[c]
                if count - 1 == 0:
                    wmap.pop(c)
                else:
                    wmap[c] -= 1
            l += 1
        if minWStart == -1:
            return ""
        return s[minWStart:minWEnd+1]
    def match(self, wmap, tmap):
        for key, val in tmap.items():
            if key not in wmap:
                return False
            if val > wmap[key]:
                return False
        return True
