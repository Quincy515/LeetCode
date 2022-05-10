struct Solution;

impl Solution {
    pub fn exchange(mut nums: Vec<i32>) -> Vec<i32> {
        if nums.is_empty() {
            return nums;
        }
        let (mut i, mut j) = (0, nums.len() - 1);
        while i < j {
            if nums[i] % 2 == 1 {
                i += 1;
                continue;
            }
            if nums[j] % 2 == 0 {
                j -= 1;
                continue;
            }
            nums.swap(i, j);
            i += 1;
            j -= 1;
        }
        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::exchange(vec![1, 2, 3, 4]), vec![1, 3, 2, 4]);
    }
}
