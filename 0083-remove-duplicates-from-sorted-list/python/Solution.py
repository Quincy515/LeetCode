# Definition for singly-linked list.
class ListNode(object):
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


class Solution(object):
    def deleteDuplicates(self, head):
        """
        :type head: ListNode
        :rtype: ListNode
        """
        if not head:
            return head
        cur = head
        next = cur.next
        while next:
            if cur.val == next.val:
                cur.next = next.next
            else:
                cur = cur.next
            next = next.next
        return head