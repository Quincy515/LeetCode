class Solution:
    def exchangeBits(self, num: int) -> int:
        ret = 0
        for i in range(0, 30, 2):
            a1 = (num & (1 << i))
            b1 = (num & (1 << (i+1)))
            if a1 != 0:
                ret |= (1<<(i+1))
            if b1 != 0:
                ret |= (1<<i)
        return ret
