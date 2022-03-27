class Solution:
    def __init__(self):
        self.result = []

    def permute(self, nums: List[int]) -> List[List[int]]:
        self.backtrack(nums, 0, [])
        return self.result

    def backtrack(self, nums, k, path):
        if k == len(nums):
            self.result.append(path[:])
            return
        for i in range(len(nums)):
            if nums[i] in path:
                continue
            path.append(nums[i])
            self.backtrack(nums, k+1,path)
            path.pop()
