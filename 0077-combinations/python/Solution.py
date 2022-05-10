class Solution:
    def __init__(self):
        self.result = []

    def combine(self, n: int, k: int) -> List[List[int]]:
        self.backtrack(n, k, 1, [])
        return self.result

    def backtrack(self, n, k, step, path):
        if len(path) == k:
            self.result.append(path)
            return
        if step == n + 1:
            return
        self.backtrack(n, k, step+1, path)
        path.append(step)
        self.backtrack(n,k,step+1, path)
        path.pop()
