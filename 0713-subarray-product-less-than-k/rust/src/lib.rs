struct Solution;

impl Solution {
    pub fn num_subarray_product_less_than_k_v1(nums: Vec<i32>, k: i32) -> i32 {
        let (mut left, mut right) = (0, 0);
        let mut result = 0;
        let mut val = i32::MAX;
        while left < nums.len() {
            if left == right && nums[left] < k {
                println!("1. {}", nums[left]);
                result += 1;
                if right < nums.len() - 1 {
                    right += 1;
                    val = nums[left] * nums[right];
                } else {
                    left += 1;
                }
            } else if val < k {
                result += 1;
                println!("2. {:?}", &nums[left..=right]);
                if right < nums.len() - 1 {
                    right += 1;
                    val *= nums[right];
                } else {
                    left += 1;
                    right = left;
                }
            } else {
                left += 1;
                right = left;
            }
        }
        result
    }

    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        if k <= 1 {
            return 0;
        }

        let (mut left, mut right) = (0, 0);
        let (mut prod, mut ans) = (1, 0);
        // 开始滑动窗口
        while right < nums.len() {
            prod *= nums[right];
            // 缩小左窗口
            while prod >= k {
                prod /= nums[left];
                left += 1;
            }
            ans += right - left + 1;
            right += 1;
        }
        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(
            Solution::num_subarray_product_less_than_k(vec![10, 5, 2, 6], 100),
            8
        );
        assert_eq!(
            Solution::num_subarray_product_less_than_k(vec![1, 2, 3], 0),
            0
        );
        assert_eq!(
            Solution::num_subarray_product_less_than_k(vec![1, 1, 1], 1),
            0
        );
    }
}
