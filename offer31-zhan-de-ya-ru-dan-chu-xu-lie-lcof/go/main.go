package main

// Time：O(n), Space: O(n)
func validateStackSequences(pushed []int, popped []int) bool {
	// 处理边界情况，如果两个序列中有一个为空，或它们的长度不相等。
	if pushed == nil || popped == nil || len(pushed) != len(popped) {
		return false // 直接返回false
	}
	var stack []int // 否则定义一个辅助栈
	p := 0          // 并初始化游标p用于指向出栈序列，当前数字
	// 接下来遍历入栈序列
	for i := range pushed {
		// 把入栈序列中的数字压入栈中
		stack = append(stack, pushed[i])
		// 如果栈不为空，并且栈顶元素等于出栈序列当前数字
		for len(stack) > 0 && stack[len(stack)-1] == popped[p] {
			// 就不断出栈，并移动游标p的操作
			stack = stack[:len(stack)-1]
			p++
		}
	}
	// 把所有数字入栈后，只要判断栈是否为空
	// 就可知道这个数组是否为合法的入栈出栈序列
	return len(stack) == 0
}

// Time：O(n), Space: O(n)
// 用数组模拟栈
func validateStackSequences(pushed []int, popped []int) bool {
	// 处理边界情况，如果两个序列中有一个为空，或它们的长度不相等。
	if pushed == nil || popped == nil || len(pushed) != len(popped) {
		return false // 直接返回false
	}
	stack := make([]int, len(pushed)) // 否则定义一个长度和入栈序列一样的数组
	top := -1                         // 使用top游标来指示栈顶位置，初始化为-1
	p := 0                            // 并初始化游标p用于指向出栈序列，当前数字
	// 接下来遍历入栈序列
	for i := range pushed {
		// 把入栈序列中的数字压入栈中
		top++                  // 栈顶元素位置
		stack[top] = pushed[i] // 给栈顶元素赋值
		// 如果栈不为空即top!=-1，并且栈顶元素stack[top]等于出栈序列当前数字
		for top != -1 && stack[top] == popped[p] {
			// 就不断出栈，并移动游标p的操作
			top-- // 出栈即top--即可。
			p++
		}
	}
	// 把所有数字入栈后，只要判断栈是否为空
	// 就可知道这个数组是否为合法的入栈出栈序列
	return top == -1 // 判断栈是否为空
}
