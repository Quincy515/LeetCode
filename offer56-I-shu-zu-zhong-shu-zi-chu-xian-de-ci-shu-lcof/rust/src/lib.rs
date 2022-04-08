struct Solution;

impl Solution {
    pub fn single_numbers(nums: Vec<i32>) -> Vec<i32> {
        let (mut xor_result, n) = (0, nums.len());
        for i in 0..n {
            xor_result ^= nums[i];
        }
        let mut tag = 1;
        while (xor_result & tag) == 0 {
            tag <<= 1;
        }
        let (mut a, mut b) = (0, 0);
        for i in 0..n {
            if nums[i] & tag == 0 {
                a ^= nums[i];
            } else {
                b ^= nums[i];
            }
        }
        vec![a, b]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(Solution::single_numbers(vec![4, 1, 4, 6]), vec![1, 6]);
        assert_eq!(
            Solution::single_numbers(vec![1, 2, 10, 4, 1, 4, 3, 3]),
            vec![2, 10]
        );
    }
}
