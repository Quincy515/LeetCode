class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        n = len(prices)
        maxPrices = [0] * n
        curMax = 0
        for i in range(n-1,-1,-1):
            maxPrices[i] = curMax
            if prices[i] > curMax:
                curMax = prices[i]
        result = 0
        for i in range(n):
            if result < maxPrices[i] - prices[i]:
                result = maxPrices[i] - prices[i]
        return result
