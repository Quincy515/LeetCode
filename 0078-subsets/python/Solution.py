class Solution:
    def __init__(self):
        self.result = []

    def subsets(self, nums: List[int]) -> List[List[int]]:
        self.backtrack(nums, 0, [])
        return self.result

    def backtrack(self, nums, k, path):
        if k == len(nums):
            self.result.append(path[:])
            return
        self.backtrack(nums, k+1,path)
        path.append(nums[k])
        self.backtrack(nums, k+1,path)
        path.pop()
