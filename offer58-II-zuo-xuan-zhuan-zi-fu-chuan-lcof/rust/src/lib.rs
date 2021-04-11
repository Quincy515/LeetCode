struct Solution {}
impl Solution {
    pub fn reverse_left_words(s: String, n: i32) -> String {
        String::from(&s[n as usize..]) + &s[..n as usize]
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
