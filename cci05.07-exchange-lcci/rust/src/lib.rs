struct Solution;

impl Solution {
    pub fn exchange_bits(num: i32) -> i32 {
        let mut ret = 0;
        let mut i = 0;
        while i <= 30 {
            // 奇位与偶位
            let a1 = num & (1 << i);
            let b1 = num & (1 << (i + 1));
            // 如果是 1，则加上交换位；0 则加上也没用
            if a1 != 0 {
                ret |= 1 << (i + 1);
            }
            if b1 != 0 {
                ret |= 1 << i;
            }
            i += 2;
        }

        ret
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(Solution::exchange_bits(2), 1);
        assert_eq!(Solution::exchange_bits(3), 3);
    }
}
