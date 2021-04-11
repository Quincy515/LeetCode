pub struct Solution {}
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        if s.is_empty() {
            return true; // 边界情况，字符串为空或者长度为0，确定是回文字符串
        }
        let seq = s.chars().collect::<Vec<_>>();
        let (mut i, mut j) = (0_usize, seq.len() - 1);
        while i < j {
            if !seq[i].is_ascii_alphanumeric() {
                // 判断是否是字母或数字
                i += 1;
                continue;
            }
            if !seq[j].is_ascii_alphanumeric() {
                j -= 1;
                continue;
            }
            if seq[i].to_ascii_lowercase() != seq[j].to_ascii_lowercase() {
                return false; // 转成小写后是否相等
            } else {
                i += 1;
                j -= 1;
            }
        }
        return true;
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
