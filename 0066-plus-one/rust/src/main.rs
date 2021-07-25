impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let mut i = digits.len() - 1;
        loop {
            // 数字非9，直接加一返回
            if digits[i] < 9 {
                digits[i] += 1;
                return digits;
            }
            // 数字是9，将其置为0
            digits[i] = 0;
            if i > 0 {
                i -= 1;
            } else if i == 0 {
                // 全部数字是9，跳出循环
                break;
            }
        }
        // 重置数组，数组长度因进位而加一，除第一元素为1外，其余元素皆为0
        digits = vec![0; digits.len() + 1];
        digits[0] = 1;
        return digits;
    }
}
