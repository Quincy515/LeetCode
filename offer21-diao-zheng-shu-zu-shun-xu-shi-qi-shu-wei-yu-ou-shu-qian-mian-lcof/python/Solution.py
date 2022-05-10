class Solution:
    def exchange(self, nums: List[int]) -> List[int]:
        i = 0
        j = len(nums) -1
        while i < j:
            if nums[i] % 2 == 1:
                i += 1
                continue
            if nums[j] % 2 == 0:
                j -= 1
                continue
            tmp = nums[i]
            nums[i] = nums[j]
            nums[j] = tmp
            i += 1
            j -= 1
        return nums