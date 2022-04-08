class Solution:
    def swapNumbers(self, numbers: List[int]) -> List[int]:
        if numbers[0] == numbers[1]:
            return numbers
        numbers[0] ^= numbers[1]
        numbers[1] ^= numbers[0]
        numbers[0] ^= numbers[1]
        return numbers
