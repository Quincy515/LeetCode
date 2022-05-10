class Solution:
    def __init__(self):
        self.result = []

    def letterCombinations(self, digits: str) -> List[str]:
        if len(digits) == 0:
            return self.result
        mappings = [None] * 10
        mappings[2] = "abc"
        mappings[3] = "def"
        mappings[4] = "ghi"
        mappings[5] = "jkl"
        mappings[6] = "mno"
        mappings[7] = "pqrs"
        mappings[8] = "tuv"
        mappings[9] = "wxyz"
        path = [None] * len(digits)
        self.backtrack(mappings, digits, 0, path)
        return self.result

    def backtrack(self, mappings, digits, k, path):
        if k == len(digits):
            self.result.append("".join(path))
            return
        mapping = mappings[int(digits[k])]
        for i in range(len(mapping)):
            path[k] = mapping[i]
            self.backtrack(mappings, digits, k+1, path)
