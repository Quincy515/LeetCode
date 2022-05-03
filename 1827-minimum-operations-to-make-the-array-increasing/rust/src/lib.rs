struct Solution;
impl Solution {
    pub fn min_operations(mut nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return 0;
        }
        let mut result = 0;
        for i in 1..nums.len() {
            if nums[i] - nums[i - 1] <= 0 {
                let temp = nums[i - 1] - nums[i] + 1;
                nums[i] += temp;
                result += temp;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(Solution::min_operations(vec![1, 1, 1]), 3);
        assert_eq!(Solution::min_operations(vec![1, 5, 2, 4, 1]), 14);
        assert_eq!(Solution::min_operations(vec![8]), 0);
    }
}
