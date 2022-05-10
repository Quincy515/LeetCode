class Solution:
    def __init__(self):
        self.result = []

    def combinationSum3(self, k: int, n: int) -> List[List[int]]:
        self.backtrack(k,n,1,0,[])
        return self.result

    def backtrack(self, k, n, step, total, path):
        if total == n and len(path) == k:
            self.result.append(path[:])
            return
        if total > n or len(path) > k or step > 9:
            return
        self.backtrack(k,n,step+1,total,path)
        path.append(step)
        self.backtrack(k,n,step+1,total+step,path)
        path.pop()
