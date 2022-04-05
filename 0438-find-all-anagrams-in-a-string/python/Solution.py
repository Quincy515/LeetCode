class Solution:
    def findAnagrams(self, s: str, p: str) -> List[int]:
        n = len(s)
        m = len(p)
        if m > n:
            return []
        needs = [0] * 26
        for i in range(m):
            needs[ord(p[i]) - ord("a")] +=1
        matched = [0] * 26
        startp = 0
        endp = 0
        result = []
        while endp < m:
            matched[ord(s[endp]) - ord("a")] += 1
            endp += 1
        if self.same(needs, matched):
            result.append(startp)
        while endp < n and startp < n:
            matched[ord(s[startp]) - ord("a")] -= 1
            matched[ord(s[endp]) - ord("a")] += 1
            startp += 1
            endp += 1
            if self.same(needs, matched):
                result.append(startp)
        return result

    def same(self, needs, matched):
        for i in range(len(needs)):
            if needs[i] != matched[i]:
                return False
        return True
