class Solution:
    def convertInteger(self, A: int, B: int) -> int:
        C = A ^ B
        count = 0
        for i in range(32):
            if (C&1) == 1:
                count += 1
            C >>= 1
        return count
