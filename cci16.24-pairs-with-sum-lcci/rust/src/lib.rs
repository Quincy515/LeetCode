struct Solution;

impl Solution {
    pub fn pair_sums(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut results = vec![];
        nums.sort();
        let (mut i, mut j) = (0, nums.len() - 1);
        while i < j {
            if nums[i] + nums[j] == target {
                let mut result = vec![];
                result.push(nums[i]);
                result.push(nums[j]);
                results.push(result);
                i += 1;
                j -= 1;
            } else if nums[i] + nums[j] < target {
                i += 1;
            } else {
                j -= 1;
            }
        }
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::pair_sums(vec![5, 6, 5], 11), vec![vec![5, 6]]);
        assert_eq!(Solution::pair_sums(vec![5, 6, 5, 6], 11), vec![vec![5, 6], vec![5, 6]]);
    }
}
