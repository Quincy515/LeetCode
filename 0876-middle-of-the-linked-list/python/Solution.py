class ListNode(object):
    def __init__(self, x):
        self.val = x
        self.next = None


class Solution(object):
    def middleNode(self, head):
        """
        :type head: ListNode
        :rtype: ListNode
        """
        if not head:
            return head
        fast = head
        slow = head
        while fast and fast.next:
            fast = fast.next.next
            slow = slow.next
        return slow
    def middleNodeTwoPass(self, head):
        if not head:
            return head
        p = head
        length = 0
        while p:
            p = p.next
            length += 1
        p = head
        for i in range(length / 2):
            p = p.next
        return p