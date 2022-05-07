struct Solution;

impl Solution {
    pub fn sum_odd_length_subarrays(mut arr: Vec<i32>) -> i32 {
        let mut result = 0;
        for i in 1..arr.len() {
            arr[i] += arr[i - 1];
        }
        for i in (1..=arr.len()).step_by(2) {
            for j in i - 1..arr.len() {
                // 以第 j 个数结尾并且长度为 i 的子数组
                let pre = if j as i32 - i as i32 == -1 {
                    0
                } else {
                    arr[j - i]
                };
                result += arr[j] - pre;
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
        assert_eq!(Solution::sum_odd_length_subarrays(vec![1, 4, 2, 5, 3]), 58);
        assert_eq!(Solution::sum_odd_length_subarrays(vec![1, 2]), 3);
        assert_eq!(Solution::sum_odd_length_subarrays(vec![10, 11, 12]), 66);
    }
}
