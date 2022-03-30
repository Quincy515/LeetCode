class Solution:
    def wordBreak(self, s: str, wordDict: List[str]) -> bool:
        n = len(s)
        dp = [False] * (n + 1)
        dp[0] = True
        for i in range(1, n+1):
            for word in wordDict:
                lenght = len(word)
                startp = i - lenght
                if startp >= 0 and s.startswith(word,startp) and dp[i - lenght]:
                    dp[i] = True
        return dp[n]
