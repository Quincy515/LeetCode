class Solution:
    def singleNumbers(self, nums: List[int]) -> List[int]:
        xorResult = 0
        n = len(nums)
        for i in range(n):
            xorResult ^= nums[i]
        tag = 1
        while xorResult & tag == 0:
            tag = tag << 1
        a = 0
        b = 0
        for i in range(n):
            if (nums[i] & tag) == 0:
                a ^= nums[i]
            else:
                b ^= nums[i]
        return [a,b]
