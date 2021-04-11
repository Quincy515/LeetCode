struct Solution {}
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let seq: Vec<char> = s.chars().rev().collect();
        let (mut result, mut find) = (0i32, false);
        for ch in seq {
            if ch == ' ' && find {
                break;
            }
            if ch != ' ' {
                find = true;
                result += 1;
            }
        }
        result
    }
    pub fn length_of_last_word_1(s: String) -> i32 {
        if s.trim().len() != 0 {
            return s.split_whitespace().last().unwrap().len() as i32;
        }
        return 0;
    }
    pub fn length_of_last_word_2(s: String) -> i32 {
        s.trim().split(" ").last().unwrap().len() as i32
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
