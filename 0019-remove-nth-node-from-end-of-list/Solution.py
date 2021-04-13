# Definition for singly-linked list.
class ListNode(object):
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


class Solution(object):
    def removeNthFromEnd(self, head, n):
        """
        :type head: ListNode
        :type n: int
        :rtype: ListNode
        """
        if not head:
            return head
        dummy = ListNode(-1)
        dummy.next = head
        p = dummy
        q = dummy

        while n > 0 and q.next:
            q = q.next
            n -= 1

        while q.next:
            p = p.next
            q = q.next

        p.next = p.next.next
        return dummy.next