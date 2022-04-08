struct Solution;
impl Solution {
    pub fn is_power_of_two(mut n: i32) -> bool {
        while n != 0 {
            if (n & 1) == 1 {
                return (n >> 1) == 0;
            }
            n >>= 1;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::is_power_of_two(1), true);
    }
}
