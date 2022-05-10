class Solution:
    def __init__(self):
        self.result = []

    def generateParenthesis(self, n: int) -> List[str]:
        path = [None] * 2 * n
        self.backtrack(n, 0, 0, 0, path)
        return self.result

    def backtrack(self, n, leftUsed, rightUesd, k, path):
        if k == 2*n:
            self.result.append("".join(path))
            return
        if leftUsed > rightUesd:
            path[k] = ")"
            self.backtrack(n, leftUsed, rightUesd+1, k+1, path)
        if leftUsed < n:
            path[k] = "("
            self.backtrack(n, leftUsed+1, rightUesd, k+1, path)
