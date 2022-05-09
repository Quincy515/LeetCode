struct Solution;

impl Solution {
    pub fn search_ans(nums: Vec<i32>, target: i32) -> i32 {
        let index = Self::bsearch(nums.clone(), target);
        if index == nums.len() as i32 || nums[index as usize] != target {
            return -1;
        }
        index
    }

    fn bsearch(nums: Vec<i32>, target: i32) -> i32 {
        let mut result = nums.len();
        if nums.is_empty() {
            return result as i32;
        }

        let (mut left, mut right) = (0, nums.len());
        while left < right {
            let mid = left + (right - left) / 2;
            if nums[mid] >= target {
                result = mid;
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        result as i32
    }

    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return -1;
        }
        let (mut left, mut right) = (0, nums.len() - 1);
        while left <= right {
            let mid = left + (right - left) / 2;
            match nums[mid].cmp(&target) {
                std::cmp::Ordering::Equal => return mid as i32,
                std::cmp::Ordering::Less => {
                    if mid + 1 > nums.len() - 1 {
                        break;
                    }
                    left = mid + 1;
                }
                std::cmp::Ordering::Greater => {
                    if mid < 1 {
                        break;
                    }
                    right = mid - 1;
                }
            }
            // if nums[mid] < target {
            //     if mid +1 >  nums.len() - 1 {
            //         break;
            //     }
            //     left = mid + 1;
            // } else if nums[mid] > target {
            //     if mid < 1 {
            //         break;
            //     }
            //     right = mid - 1;
            // } else {
            //     return mid as i32;
            // }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_should_work() {
        let nums = vec![-1, 0, 3, 5, 9, 12];
        assert_eq!(Solution::search_ans(nums, 9), 4);

        let nums = vec![5];
        assert_eq!(Solution::search(nums, -5), -1);
    }
}
