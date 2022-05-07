struct Solution;
impl Solution {
    pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
        let mut hash = std::collections::HashMap::new();
        let mut result = vec![];
        for num in nums {
            *hash.entry(num).or_insert(0) += 1;
        }
        for (k, v) in hash {
            if v == 2 {
                result.push(k);
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
        assert_eq!(
            Solution::find_duplicates(vec![4, 3, 2, 7, 8, 2, 3, 1]),
            vec![2, 3]
        );
        assert_eq!(Solution::find_duplicates(vec![1, 1, 2]), vec![1]);
        assert_eq!(Solution::find_duplicates(vec![1]), vec![]);
    }
}
