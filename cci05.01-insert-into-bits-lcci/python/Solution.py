class Solution:
    def insertBits(self, N: int, M: int, i: int, j: int) -> int:
        nbits = [0] * 32
        mbits = [0] * 32
        nbitsNum = 0
        mbitsNum = 0
        mask = 1
        for k in range(32):
            if (N & mask) != 0:
                nbits[k] = 1
            mask <<=1
        mask = 1
        for k in range(32):
            if (M&mask) != 0:
                mbits[k] = 1
            mask <<= 1
        for k in range(i,j+1):
            nbits[k] = mbits[k-i]
        mask = 1
        ret = 0
        for k in range(32):
            ret += (nbits[k] * mask)
            mask <<= 1
        return ret
