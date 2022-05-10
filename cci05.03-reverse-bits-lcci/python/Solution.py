class Solution:
    def reverseBits(self, num: int) -> int:
        if num == 0:
            return 1
        nums = [None] * 32
        for i in range(32):
            nums[i] = num & 1
            num >>= 1
        leftOneCounts = [0] * 32
        count = 0
        for i in range(32):
            leftOneCounts[i] = count
            if nums[i] == 1:
                count += 1
            else:
                count = 0
        rightOneCounts = [0] * 32
        count = 0
        for i in range(31, -1,-1):
            rightOneCounts[i] = count
            if nums[i] == 1:
                count += 1
            else:
                count = 0
        ret = 0
        for i in range(32):
            if ret < leftOneCounts[i] + rightOneCounts[i] + 1:
                ret = leftOneCounts[i] + rightOneCounts[i] + 1
        return ret
