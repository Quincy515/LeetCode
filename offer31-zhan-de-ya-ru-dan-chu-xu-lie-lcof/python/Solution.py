class Solution(object):
    def validateStackSequences(self, pushed, popped):
        """
        :type pushed: List[int]
        :type popped: List[int]
        :rtype: bool
        """
        stack = []
        pop_index = 0
        for num in pushed:
            stack.append(num)
            while stack and stack[-1] == popped[pop_index]:
                stack.pop()
                pop_index += 1

        if len(stack) == 0:
            return True
        else:
            return False