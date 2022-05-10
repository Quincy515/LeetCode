class Solution:
    def productExceptSelf(self, nums: List[int]) -> List[int]:
        n = len(nums)
        leftProducts = [0] * n
        rightProducts = [0] * n
        product = 1
        for i in range(n):
            product *= nums[i]
            leftProducts[i] = product
        product = 1
        for i in range(n-1,-1,-1):
            product *= nums[i]
            rightProducts[i] = product
        result = [None] * n
        for i in range(n):
            result[i] = 1
            if i - 1 >= 0:
                result[i] *= leftProducts[i-1]
            if i + 1 < n:
                result[i] *= rightProducts[i+1]
        return result

class Solution:
    def productExceptSelf(self, nums: List[int]) -> List[int]:
        n = len(nums)
        result = [None] * n
        leftProduct = 1
        for i in range(n):
            result[i] = leftProduct
            leftProduct *= nums[i]
        rightProduct = 1
        for i in range(n-1,-1,-1):
            result[i] *= rightProduct
            rightProduct *= nums[i]
        return result
