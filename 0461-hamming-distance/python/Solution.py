class Solution:
    def hammingDistance(self, x: int, y: int) -> int:
        r = x ^ y
        mask = 1
        count = 0
        for i in range(31):
            if (r & mask) != 0:
                count += 1
            mask *= 2
        return count
