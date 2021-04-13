# Definition for singly-linked list.
class ListNode(object):
    def __init__(self, x):
        self.val = x
        self.next = None


class Solution(object):
    def getIntersectionNode_without_len(self, headA, headB):
        """
        :type head1, head1: ListNode
        :rtype: ListNode
        """
        p = headA
        q = headB
        while p != q:
            p = headB if not p else p.next
            q = headA if not q else q.next
        return p

    def getIntersectionNode_with_len(self, headA, headB):
        """
        :type head1, head1: ListNode
        :rtype: ListNode
        """
        tmp1 = headA
        length1 = 0
        tmp2 = headB
        length2 = 0
        while tmp1:
            length1 += 1
            tmp1 = tmp1.next
        while tmp2:
            length2 += 1
            tmp2 = tmp2.next
        diff = length1 - length2
        longer = headA
        shorter = headB
        if diff < 0:
            diff = abs(diff)
            longer = headB
            shorter = headA
        for i in range(diff):
            longer = longer.next
        while longer != shorter:
            longer = longer.next
            shorter = shorter.next
        return longer