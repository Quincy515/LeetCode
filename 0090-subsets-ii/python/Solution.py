class Solution:
    def __init__(self):
        self.result = []

    def subsetsWithDup(self, nums: List[int]) -> List[List[int]]:
        hm = dict()
        for i in range(len(nums)):
            count = 1
            if nums[i] in hm:
                count += hm[nums[i]]
            hm[nums[i]] = count
        n = len(hm)
        uniqueNum = [None] * n
        counts = [None] * n
        k = 0
        for i in range(len(nums)):
            if nums[i] in hm:
                uniqueNum[k] = nums[i]
                counts[k] = hm[nums[i]]
                k +=1
                hm.pop(nums[i])
        self.backtrack(uniqueNum, counts, 0, [])
        return self.result

    def backtrack(self, uniqueNum, counts, k, path):
        if k == len(uniqueNum):
            self.result.append(path[:])
            return
        for count in range(counts[k]+1):
            for i in range(count):
                path.append(uniqueNum[k])
            self.backtrack(uniqueNum, counts, k+1, path)
            for i in range(count):
                path.pop()
