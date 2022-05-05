struct Solution;
impl Solution {
    pub fn minimum_difference(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort();
        let (mut i, mut j) = (0i32, -1i32);
        let mut result = i32::MAX;
        while j < nums.len() as i32 - 1 {
            j += 1;
            if j - i + 1 > k {
                i += 1;
            }
            if j - i + 1 == k {
                result = result.min(nums[j as usize] - nums[i as usize]);
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
        assert_eq!(Solution::minimum_difference(vec![90], 1), 0);
        assert_eq!(Solution::minimum_difference(vec![9, 4, 1, 7], 2), 2);
    }
}
