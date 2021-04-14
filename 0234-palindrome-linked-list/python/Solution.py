# Definition for singly-linked list.
class ListNode(object):
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


class Solution(object):
    def isPalindrome(self, head):
        """
        :type head: ListNode
        :rtype: bool
        """
        if not head:
            return True
        stack = []
        queue = []
        size = 0
        tmp = head
        while tmp:
          
            stack.append(tmp.val)
            queue.append(tmp.val)
            size += 1
            tmp = tmp.next
        for i in range(size):
            s_result = stack.pop()
            q_result = queue.pop()
            if s_result != q_result:
                return False
        return True