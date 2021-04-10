# coding=utf-8

class Solution(object):
    def canJump(self, nums):
        """
        :type nums: List[int]
        :rtype: bool
        """
        if not nums:
            return True
        max_v = 0
        length = len(nums)
        for i in range(length):
            if max_v >= length - 1:
                return True
            if i > max_v:
                return False
            max_v = max(max_v, i + nums[i])
        return False
