struct Solution;

impl Solution {
    pub fn reverse_bits(mut num: i32) -> i32 {
        if num == 0 {
            return 1;
        }
        let mut nums = vec![0; 32];
        for i in 0..32 {
            nums[i] = num & 1;
            num >>= 1;
        }
        let mut left_one_counts = vec![0; 32];
        let mut count = 0;
        for i in 0..32 {
            left_one_counts[i] = count;
            if nums[i] == 1 {
                count += 1;
            } else {
                count = 0;
            }
        }
        let mut right_one_counts = vec![0; 32];
        let mut count = 0;
        for i in (0..=31).rev() {
            right_one_counts[i] = count;
            if nums[i] == 1 {
                count += 1;
            } else {
                count = 0;
            }
        }

        let mut ret = 0;
        for i in 0..32 {
            if ret < left_one_counts[i] + right_one_counts[i] + 1 {
                ret = left_one_counts[i] + right_one_counts[i] + 1;
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(Solution::reverse_bits(1775), 8);
        assert_eq!(Solution::reverse_bits(7), 4);
    }
}
