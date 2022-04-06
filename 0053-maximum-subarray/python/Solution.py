class Solution:
    def maxSubArray(self, nums: List[int]) -> int:
        n = len(nums)
        maxSum = float("-inf")
        total = 0
        for i in range(n):
            if total < 0:
                total = 0
            total += nums[i]
            if total > maxSum:
                maxSum = total
        return maxSum


class Solution:
    def maxSubArray(self, nums: List[int]) -> int:
        if len(nums) == 1:
            return nums[0]
        n = len(nums)
        totalSum = [0] * n
        maxCur = [0] * n
        cursum = 0
        for i in range(n):
            cursum += nums[i]
            totalSum[i]= cursum
        cursum = float("-inf")
        for i in range(n-1, -1,-1):
            if cursum < totalSum[i]:
                cursum = totalSum[i]
            maxCur[i] = cursum
        result = float("-inf")
        for i in range(n):
            if i == 0 and result < maxCur[0]:
                result = maxCur[0]
            if i != 0 and result < maxCur[i] - totalSum[i-1]:
                result = maxCur[i] - totalSum[i-1]
        return result
