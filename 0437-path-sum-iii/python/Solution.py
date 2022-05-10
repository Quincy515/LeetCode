# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

class Solution:
    def __init__(self):
        self.count = 0

    def pathSum(self, root: TreeNode, targetSum: int) -> int:
        self.dfs(root, targetSum)
        return self.count

    def dfs(self, root, targetSum):
        if not root:
            return {}
        leftValues = self.dfs(root.left, targetSum)
        rightValues = self.dfs(root.right, targetSum)
        rootValues = dict()
        rootValues[root.val] = 1
        for key, value in leftValues.items():
            key = key + root.val
            if key in rootValues:
                value += rootValues[key]
            rootValues[key] = value
        for key, value in rightValues.items():
            key = key + root.val
            if key in rootValues:
                value += rootValues[key]
            rootValues[key] = value
        for key, value in rootValues.items():
            if key == targetSum:
                self.count += value
        return rootValues


class Solution:
    def __init__(self):
        self.count = 0
    def pathSum(self, root: TreeNode, targetSum: int) -> int:
        self.dfs(root,[], targetSum)
        return self.count
    def dfs(self, root, path, targetSum):
        if not root:
            return
        path.append(root.val)
        curSum = 0
        for i in range(len(path)-1, -1,-1):
            curSum += path[i]
            if curSum == targetSum:
                self.count += 1
        self.dfs(root.left, path, targetSum)
        self.dfs(root.right, path, targetSum)
        path.pop()