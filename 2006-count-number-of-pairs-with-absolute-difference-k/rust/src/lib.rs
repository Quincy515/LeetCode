struct Solution;
impl Solution {
    pub fn count_k_difference(nums: Vec<i32>, k: i32) -> i32 {
        let mut hash = std::collections::HashMap::new();
        let mut result = 0;

        for num in nums {
            let mut x = num + k;
            result += *hash.entry(x).or_insert(0);
            x = num - k;
            result += *hash.entry(x).or_insert(0);
            *hash.entry(num).or_insert(0) += 1;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::count_k_difference(vec![1, 2, 2, 1], 1), 4);
        assert_eq!(Solution::count_k_difference(vec![1, 3], 3), 0);
        assert_eq!(Solution::count_k_difference(vec![3, 2, 1, 5, 4], 2), 3);
    }
}
