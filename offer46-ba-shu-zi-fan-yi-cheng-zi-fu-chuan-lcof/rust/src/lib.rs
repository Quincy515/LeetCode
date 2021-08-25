struct Solution;

impl Solution {
    pub fn translate_num(num: i32) -> i32 {
        if num >= 10 {
            (if 10 <= num % 100 && num % 100 <= 25 {
                Solution::translate_num(num / 100)    
            } else {0}) + Solution::translate_num(num / 10) 
        } else {1}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(5, Solution::translate_num(12258));
    }
}
