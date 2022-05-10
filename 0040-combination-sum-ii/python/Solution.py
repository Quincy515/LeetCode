class Solution:
    def __init__(self):
        self.result = []

    def combinationSum2(self, candidates: List[int], target: int) -> List[List[int]]:
        hashTable = dict()
        for i in range(len(candidates)):
            if candidates[i] not in hashTable:
                hashTable[candidates[i]] = 1
            else:
                hashTable[candidates[i]] += 1
        nums = []
        counts = []
        for i in range(len(candidates)):
            if candidates[i] in hashTable:
                nums.append(candidates[i])
                counts.append(hashTable[candidates[i]])
                hashTable.pop(candidates[i])

        self.backtrack(nums, counts, 0, target, [])
        return self.result

    def backtrack(self, nums, counts, k, left, path):
        if left == 0:
            self.result.append(path[:])
            return
        if k == len(nums) or left < 0:
            return
        for count in range(counts[k] +1):
            for i in range(count):
                path.append(nums[k])
            self.backtrack(nums, counts, k+1, left - count*nums[k], path)
            for i in range(count):
                path.pop()