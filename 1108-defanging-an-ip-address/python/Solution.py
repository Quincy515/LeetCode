class Solution:
    def defangIPaddr(self, address: str) -> str:
        ans = []
        for i in range(len(address)):
            if address[i] != '.':
                ans.append(address[i])
            else:
                ans.append("[.]")
        return "".join(ans)
