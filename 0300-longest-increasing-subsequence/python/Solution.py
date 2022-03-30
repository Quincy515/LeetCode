class Solution:
    def lengthOfLIS(self, nums: List[int]) -> int:
        n = len(nums)
        dp = [0] * n
        dp[0] = 1
        for i in range(1,n):
            dp[i] = 1
            for j in range(i):
                if nums[i] > nums[j]:
                    dp[i] = max(dp[i], dp[j] + 1)
        result = 0
        for i in range(n):
            if dp[i] > result:
                result = dp[i]
        return result

class Solution:
    def lengthOfLIS(self, nums: List[int]) -> int:
        n = len(nums)
        listToMinV = [float(inf)] * (n+1)
        k = 0
        dp = [0] * n
        for i in range(n):
            length = self.bsearch(listToMinV,k, nums[i])
            if length == -1:
                dp[i] = 1
            else:
                dp[i] = length+1
            if dp[i] > k:
                k = dp[i]
                listToMinV[dp[i]] = nums[i]
            elif listToMinV[dp[i]] > nums[i]:
                listToMinV[dp[i]] = nums[i]
        result = 0
        for i in range(n):
            if dp[i] > result:
                result = dp[i]
        return result

    def bsearch(self, a, k, target):
        low = 1
        high = k
        while low <= high:
            mid = (low + high) // 2
            if a[mid] < target:
                if mid == k or a[mid+1] >= target:
                    return mid
                else:
                    low = mid + 1
            else:
                high = mid - 1
        return -1