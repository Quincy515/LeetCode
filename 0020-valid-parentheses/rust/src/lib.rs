struct Solution;

impl Solution {
    pub fn is_valid_v1(s: String) -> bool {
        // 当字符串长度为奇数的时候，属于无效情况，直接返回 false
        if s.len() % 2 == 1 {
            // 无效情况，返回 false
            return false;
        }

        // 构建一个栈，用来存储括号
        let mut stack: Vec<char> = Vec::new();
        // 把字符串装换为数组的形式，方便获取字符串中的每个字符
        let char_array: Vec<char> = s.chars().collect();

        // 遍历字符串数组中的所有元素
        for i in 0..char_array.len() {
            // 获取此时的字符
            let c = char_array[i];
            // 如果字符为左括号 (, 那么就在栈中添加对左括号
            if c == '(' {
                // 添加对应左括号 (
                stack.push('(');
                // 如果字符为左括号 [, 那么就在栈中添加对左括号 [
            } else if c == '[' {
                // 添加对应的左括号 [
                stack.push('[');
                // 如果字符为左括号 {, 那么就在栈中添加对左括号 {
            } else if c == '{' {
                // 添加对应左括号 {
                stack.push('{');
                // 否则的话，说明此时 c 是 ) ] } 这三种符号的一种
            } else {
                // 如果栈已经为空，而现在遍历的字符 c 是 ) ] } 这三种符号的一种
                // 找不到可以匹配的括号，返回 false
                // 比如这种情况 }{, 直接从右括号开始，此时栈为空
                if stack.is_empty() {
                    return false;
                }
                // 如果栈不为空，获取栈顶元素
                let top = stack[stack.len() - 1];
                // 将栈顶元素和此时的元素 c 进行比较，如果可以发生闭合，则将栈顶元素移除
                if top == '(' && c == ')' || top == '[' && c == ']' || top == '{' && c == '}' {
                    // 移除栈顶元素
                    stack.pop();
                } else {
                    // 如果不相同，说明不匹配，返回 false
                    return false;
                }
            }
        }
        // 遍历完整字符数组，判断栈是否为空
        // 如果栈为空，说明字符数组中的所有括号都是闭合的
        // 如果栈不为空，说明有未闭合的括号
        stack.is_empty()
    }
}

impl Solution {
    pub fn is_valid_v2(s: String) -> bool {
        let chars: Vec<char> = s.chars().collect();
        // 判断是否为空字符串，空字符串视为有效字符串
        if chars.len() == 0 {
            return true;
        }
        let mut stack: Vec<char> = Vec::new();
        for i in 0..chars.len() {
            // 如果是左小括号，将右小括号入栈
            if chars[i] == '(' {
                stack.push(')');
            }
            // 如果是左中括号，将右中括号入栈
            else if chars[i] == '[' {
                stack.push(']');
            }
            // 如果是左大括号，将右大括号入栈
            else if chars[i] == '{' {
                stack.push('}');
            }
            // 栈为空或与栈顶元素不相同为无效字符串
            else if stack.is_empty() || chars[i] != stack.pop().unwrap() {
                return false;
            }
        }
        // 匹配结束，栈为空代表是有效字符串，否则是无效字符串
        return stack.is_empty();
    }
}

impl Solution {
    pub fn is_valid_v3(s: String) -> bool {
        let s: Vec<char> = s.chars().collect();
        let mut stack = vec![];
        for i in 0..s.len() {
            let c = s[i];
            if c == '(' || c == '[' || c == '{' {
                stack.push(c)
            } else {
                if stack.len() == 0 {
                    return false;
                }
                if stack.len() > 0 {
                    let top_char = stack.pop().unwrap();
                    if c == ')' && top_char != '(' {
                        return false;
                    }
                    if c == ']' && top_char != '[' {
                        return false;
                    }
                    if c == '}' && top_char != '{' {
                        return false;
                    }
                }
            }
        }
        stack.len() == 0
    }

    pub fn is_valid_v4(s: String) -> bool {
        let mut stack = "".to_string();
        for ch in s.chars() {
            match ch {
                '(' => stack.push(')'),
                '[' => stack.push(']'),
                '{' => stack.push('}'),
                ')' | ']' | '}' => {
                    if stack.pop() != Some(ch) {
                        return false;
                    }
                }
                _ => return false,
            }
        }
        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(true, Solution::is_valid_v1("()".to_owned()));
        assert_eq!(true, Solution::is_valid_v2("()[]{}".to_owned()));
        assert_eq!(false, Solution::is_valid_v3("(]".to_owned()));
        assert_eq!(false, Solution::is_valid_v4("([)]".to_owned()));
        assert_eq!(true, Solution::is_valid_v4("{[]}".to_owned()));
    }
}
