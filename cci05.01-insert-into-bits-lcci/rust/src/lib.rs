struct Solution;

impl Solution {
    pub fn insert_bits(n: i32, m: i32, i: i32, j: i32) -> i32 {
        let (mut nbits, mut mbits) = (vec![0; 32], vec![0; 32]);
        let mut mask = 1;
        for k in 0..32 {
            if (n & mask) != 0 {
                nbits[k] = 1;
            }
            mask <<= 1;
        }
        mask = 1;
        for k in 0..32 {
            if (m & mask) != 0 {
                mbits[k] = 1;
            }
            mask <<= 1;
        }
        for k in i..=j {
            nbits[k as usize] = mbits[(k - i) as usize];
        }
        mask = 1;
        let mut ret = 0;
        for k in 0..32 {
            ret += nbits[k] * mask;
            mask <<= 1;
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::insert_bits(1024, 19, 2, 6), 1100);
        assert_eq!(Solution::insert_bits(0, 31, 0, 4), 31);
    }
}
