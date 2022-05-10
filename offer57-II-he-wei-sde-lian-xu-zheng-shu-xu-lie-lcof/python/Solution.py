class Solution:
    def findContinuousSequence(self, target: int) -> List[List[int]]:
        result = []
        p = 1
        q = 2
        total = 3
        while p < q:
            arr = [0] * (q - p + 1)
            if total == target:
                for i in range(p, q+1):
                    arr[i-p] = i
                result.append(arr)
                total -= p
                p += 1
                q += 1
                total += q
            elif total > target:
                total -= p
                p += 1
            else:
                q += 1
                total += q
        return result
