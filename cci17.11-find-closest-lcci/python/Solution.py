class Solution:
    def findClosest(self, words: List[str], word1: str, word2: str) -> int:
        w1ps = []
        w2ps = []
        for i in range(len(words)):
            if words[i] == word1:
                w1ps.append(i)
            elif words[i] == word2:
                w2ps.append(i)
        p1 = 0
        p2 = 0
        minRet = float("inf")
        while p1 < len(w1ps) and p2 < len(w2ps):
            pos1 = w1ps[p1]
            pos2 = w2ps[p2]
            if pos1 > pos2:
                if minRet > pos1 - pos2:
                    minRet = pos1 - pos2
                p2 += 1
            else:
                if minRet > pos2 - pos1:
                    minRet = pos2 - pos1
                p1 += 1
        return minRet