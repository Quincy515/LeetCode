class Solution:
    def __init__(self):
        self.result = []

    def permuteUnique(self, nums: List[int]) -> List[List[int]]:
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
        self.backtrack(uniqueNum, counts, 0, [], len(nums))
        return self.result

    def backtrack(self, uniqueNum, counts, k, path,n):
        if k == n:
            self.result.append(path[:])
            return
        for i in range(len(uniqueNum)):
            if counts[i] == 0:
                continue
            path.append(uniqueNum[i])
            counts[i] -= 1
            self.backtrack(uniqueNum, counts, k+1, path, n)
            path.pop()
            counts[i] += 1
