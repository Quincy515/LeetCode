class Solution:
    def trap(self, height: List[int]) -> int:
        n = len(height)
        result = 0
        for i in range(1, n-1):
            lh = 0
            for j in range(i):
                if height[j] > lh:
                    lh = height[j]
            rh = 0
            for j in range(i+1,n):
                if height[j] > rh:
                    rh = height[j]
            carry = min(lh, rh) - height[i]
            if carry < 0:
                carry = 0
            result += carry
        return result

class Solution:
    def trap(self, height: List[int]) -> int:
        n = len(height)
        leftMax = [None] * n
        curMax = 0
        for i in range(n):
            leftMax[i] = max(curMax, height[i])
            curMax = leftMax[i]
        rightMax = [None] * n
        curMax = 0
        for i in range(n-1,-1,-1):
            rightMax[i] = max(curMax, height[i])
            curMax = rightMax[i]
        result = 0
        for i in range(1,n-1):
            result += min(leftMax[i], rightMax[i]) - height[i]
        return result
