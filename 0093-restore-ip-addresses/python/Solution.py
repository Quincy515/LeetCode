class Solution:
    def __init__(self):
        self.result = []

    def restoreIpAddresses(self, s: str) -> List[str]:
        self.backtrack(s, 0,0, [])
        return self.result

    def backtrack(self, s, k, step, path):
        if step == 4 and k == len(s):
            sb = [str(item) for item in path]
            self.result.append(".".join(sb))
            return
        if step > 4:
            return
        if k == len(s):
            return
        val = 0
        if k < len(s):
            val = val * 10 + int(s[k])
            path.append(val)
            self.backtrack(s, k+1, step+1, path)
            path.pop()
        if s[k] == "0":
            return
        if k + 1 < len(s):
            val = val * 10 + int(s[k+1])
            path.append(val)
            self.backtrack(s, k+2, step+1, path)
            path.pop()
        if k + 2 < len(s):
            val = val * 10 + int(s[k+2])
            if val <= 255:
                path.append(val)
                self.backtrack(s, k+3, step+1, path)
                path.pop()
