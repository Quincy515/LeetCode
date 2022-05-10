class Solution:
    def isPowerOfTwo(self, n: int) -> bool:
        while n != 0:
            if (n & 1) == 1:
                if (n >> 1) == 0:
                    return True
                else:
                    return False
            n >>= 1
        return False
