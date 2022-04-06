struct Solution;

impl Solution {
    pub fn convert_integer(a: i32, b: i32) -> i32 {
        let (mut c, mut count) = (a ^ b, 0);
        for _ in 0..32 {
            if c & 1 == 1 {
                count += 1;
            }
            c >>= 1;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(Solution::convert_integer(29, 15), 2);
        assert_eq!(Solution::convert_integer(1, 2), 2);
    }
}
