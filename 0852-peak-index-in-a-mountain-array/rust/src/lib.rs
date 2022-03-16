impl Solution {
    pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
        let (mut left, mut right) = (1, arr.len() - 2);
        while left < right {
            let mid = left + (right - left) / 2;
            if arr[mid] > arr[mid - 1] && arr[mid] > arr[mid + 1] {
                return mid as i32;
            }
            if arr[mid] > arr[mid - 1] && arr[mid] < arr[mid + 1] {
                left = mid + 1;
            }
            if arr[mid] < arr[mid - 1] && arr[mid] > arr[mid + 1] {
                right = mid - 1;
            }
        }
        left as i32
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(Solution::peak_index_in_mountain_array(vec![0, 1, 0]), 1);
        assert_eq!(Solution::peak_index_in_mountain_array(vec![0, 2, 1, 0]), 1);
        assert_eq!(Solution::peak_index_in_mountain_array(vec![0, 10, 5, 2]), 1);
        assert_eq!(Solution::peak_index_in_mountain_array(vec![3, 4, 5, 1]), 2);
        assert_eq!(
            Solution::peak_index_in_mountain_array(vec![24, 69, 100, 99, 79, 78, 67, 36, 26, 19]),
            2
        );
    }
}
