class Solution:
    def translateNum(self, num: int) -> int:
        if num <= 9:
            return 1
        digitlist = []
        while num != 0:
            digitlist.append(num % 10)
            num = num // 10
        n = len(digitlist)
        digits = []
        for i in range(n):
            digits.append(digitlist[n-i-1])
        dp = [0] * (n + 1)
        dp[0] = 1
        for i in range(1,n+1):
            dp[i] = dp[i-1]
            if i - 2 >= 0 and self.isValid(digits[i-2], digits[i-1]):
                dp[i] += dp[i-2]
        return dp[n]

    def isValid(self,a,b):
        if a == 1:
            return True
        if a == 2 and 0 <= b <= 5:
            return True
        return False
    