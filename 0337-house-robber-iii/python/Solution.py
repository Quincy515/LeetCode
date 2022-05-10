# Definition for a binary tree node.
class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

class Solution:
    def rob(self, root: TreeNode) -> int:
        money = self.postorder(root)
        return max(money[0], money[1])

    def postorder(self, root):
        if not root:
            return [0,0]
        leftMoney = self.postorder(root.left)
        rightMoney = self.postorder(root.right)
        money = [0, 0]
        money[0] = max(leftMoney[0], leftMoney[1]) + max(rightMoney[0], rightMoney[1])
        money[1] = leftMoney[0] + rightMoney[0] + root.val
        return money