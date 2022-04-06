class Solution:
    def hammingWeight(self, n: int) -> int:
        oneCount = 0
        mask = 1
        for i in range(32):
            if n & mask != 0:
                oneCount += 1
            mask <<= 1
        return oneCount

class Solution:
    def hammingWeight(self, n: int) -> int:
        oneCount = 0
        i = 1
        while n != 0:
            if (n & 1) == 1:
                oneCount += 1
            n >>= 1
        return oneCount
