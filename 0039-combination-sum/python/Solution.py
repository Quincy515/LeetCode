class Solution:
    def __init__(self):
        self.result = []

    def combinationSum(self, candidates: List[int], target: int) -> List[List[int]]:
        self.backtrack(candidates,0,target,[])
        return self.result

    def backtrack(self, candidates, k, left, path):
        if left == 0:
            self.result.append(path[:])
            return
        if k == len(candidates):
            return
        for i in range(left //candidates[k] +1):
            for j in range(i):
                path.append(candidates[k])
            self.backtrack(candidates, k+1, left - candidates[k] * i, path)
            for j in range(i):
                path.pop()
