class Solution:
    def smallestDifference(self, a: List[int], b: List[int]) -> int:
        a.sort()
        b.sort()
        n = len(a)
        m = len(b)
        minRet = float("inf")
        i = 0
        j = 0
        while i < n and j < m:
            if a[i] >= b[j]:
                minRet = min(minRet, a[i] - b[j])
                j += 1
            else:
                minRet = min(minRet, b[j] - a[i])
                i += 1
        return minRet