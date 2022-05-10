class Solution:
    def missingNumber(self, nums: List[int]) -> int:
        n = len(nums)
        ret = 0
        for i in range(n+1):
            ret ^= i
        for i in range(n):
            ret ^= nums[i]
        return ret
