class Solution:
    def __init__(self):
        self.result = []

    def partition(self, s: str) -> List[List[str]]:
        self.backtrack(s, 0, [])
        return self.result

    def backtrack(self, s, k, path):
        if k == len(s):
            self.result.append(path[:])
            return
        for end in range(k, len(s)):
            if self.ispalindrome(s, k, end):
                path.append(s[k:end+1])
                self.backtrack(s, end+1, path)
                path.pop()

    def ispalindrome(self, s, p, r):
        i = p
        j = r
        while i <= j:
            if s[i] !=s[j]:
                return False
            i += 1
            j -= 1
        return True
