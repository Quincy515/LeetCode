struct Solution;

impl Solution {
    pub fn hammingWeight(n: u32) -> i32 {
        let (mut one_count, mut mask) = (0, 1);
        for i in 0..32 {
            if (n & mask) != 0 {
                one_count += 1;
            }
            mask <<= 1;
        }
        one_count
    }
    pub fn hammingWeight2(mut n: u32) -> i32 {
        let mut one_count = 0;
        let mut i = 1;
        while n != 0 {
            if n & 1 == 1 {
                one_count += 1;
            }
            n >>= 1;
        }
        one_count
    }
    pub fn hammingWeight3(mut n: u32) -> i32 {
        let mut result = 0;
        while n != 0 {
            result += (n & 1) as i32;
            n >>= 1;
        }
        result
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::hammingWeight2(00000000000000000000000000001011),
            3
        );
        assert_eq!(
            Solution::hammingWeight2(00000000000000000000000010000000),
            1
        );
    }
}
