class Solution:
    def pairSums(self, nums: List[int], target: int) -> List[List[int]]:
        result = []
        if len(nums) == 0:
            return result
        nums.sort()
        i = 0
        j = len(nums) - 1
        while i < j:
            if nums[i] + nums[j] == target:
                result.append([nums[i], nums[j]])
                i += 1
                j -= 1
            elif nums[i] + nums[j] < target:
                i += 1
            else:
                j -= 1
        return result
