struct Solution;

impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        let (r, mut mask, mut count) = ((x ^ y) as i64, 1i64, 0);
        for _ in 0..31 {
            if (r & mask) != 0 {
                count += 1;
            }
            mask *= 2;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::hamming_distance(1, 4), 2);
        assert_eq!(Solution::hamming_distance(3, 1), 1);
    }
}
