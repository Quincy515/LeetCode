package main

func isValid(s string) bool {
	var stack []rune
	for i := 0; i < len(s); i++ {
		c := s[i]
		if c == '(' || c == '[' || c == '{' {
			stack = append(stack, rune(c))
		} else {
			if len(stack) == 0 {
				return false // 如果第一次压入栈的是右括号，没有元素进行匹配
			}
			if len(stack) > 0 {
				topChar := stack[len(stack)-1] // 获取栈顶元素
				stack = stack[:len(stack)-1]
				if c == ')' && topChar != '(' {
					return false
				}
				if c == ']' && topChar != '[' {
					return false
				}
				if c == '}' && topChar != '{' {
					return false
				}
			}
		}
	}
	return len(stack) == 0 // 判断栈是否为空
}
