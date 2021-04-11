struct Solution {}

impl Solution {
    pub fn is_palindrome_1(x: i32) -> bool {
        if x >= 0 && x < 10 {
            return true;
        }
        if x.to_string().chars().rev().collect::<String>() == x.to_string() {
            true
        } else {
            false
        }
    }
    pub fn is_palindrome(x: i32) -> bool {
        if x >= 0 && x < 10 {
            return true;
        }
        if x.signum() == -1 {
            return false;
        }
        let mut digits: Vec<i32> = Vec::new();
        let mut input = x;
        while input != 0 {
            digits.push(input % 10);
            input = input / 10;
        }
        let mut i = 0;
        while i < digits.len() / 2 {
            if digits[i] != digits[digits.len() - 1 - i] {
                return false;
            }
            i += 1;
        }
        true
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
