struct Solution {}
impl Solution {
    pub fn reverse_left_words(s: String, n: i32) -> String {
        String::from(&s[n as usize..]) + &s[..n as usize]
    }

    pub fn reverse_left_words_2(s: String, n: i32) -> String {
        use std::iter::FromIterator;

        let mut str: Vec<char> = s.chars().collect();
        for _ in 0..n {
            // 移动 n 次，每次左移 1 位
            let tmp = str[0];
            for j in 1..str.len() {
                str[j - 1] = str[j];
            }
            str[s.len() - 1] = tmp;
        }
        String::from_iter(str)
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
