struct Solution;
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut bits = vec![0; 32];
        let mut mask = 1;
        for i in 0..32 {
            for j in 0..n {
                if (nums[j] & mask) != 0 {
                    bits[i] = (bits[i] + 1) % 3;
                }
            }
            mask <<= 1;
        }
        let mut result = 0;
        mask = 1;
        for i in 0..32 {
            if bits[i] == 1 {
                result += mask;
            }
            mask <<= 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::single_number(vec![3, 4, 3, 3]), 4);
        assert_eq!(Solution::single_number(vec![9, 1, 7, 9, 7, 9, 7]), 1);
    }
}
