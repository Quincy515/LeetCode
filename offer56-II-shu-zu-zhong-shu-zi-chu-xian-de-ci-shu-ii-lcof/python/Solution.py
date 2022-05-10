class Solution:
    def singleNumber(self, nums: List[int]) -> int:
        n = len(nums)
        bits = [0] * 32
        mask = 1
        for i in range(32):
            for j in range(n):
                if (nums[j]&mask) != 0:
                    bits[i] = (bits[i] + 1) % 3
            mask <<= 1
        result = 0
        mask = 1
        for i in range(32):
            if bits[i] == 1:
                result += mask
            mask <<= 1
        return result
