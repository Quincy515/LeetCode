impl Solution {
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let (n, k) = (arr.len(), k as usize);
        let (mut left, mut right) = (0, n - k);
        while left < right {
            let mid = left + (right - left) / 2;
            if x - arr[mid] > arr[mid + k] - x {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        arr[left..left + k].to_vec()
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::find_closest_elements(vec![1, 2, 3, 4, 5], 4, 3),
            vec![1, 2, 3, 4]
        );
        assert_eq!(
            Solution::find_closest_elements(vec![1, 2, 3, 4, 5], 4, -1),
            vec![1, 2, 3, 4]
        );
    }
}
