struct Solution;
impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut hash = std::collections::HashMap::new();
        for i in 0..nums.len() {
            let count = hash.entry(nums[i as usize]).or_insert(0);
            println!("i: {i}, nums[i]: {}, {count}", nums[i]);
            result += *count; // 查找
            println!("result: {result}");
            *count += 1; // 插入
        }

        result
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::num_identical_pairs(vec![1, 2, 3, 1, 1, 3]), 4);
        assert_eq!(Solution::num_identical_pairs(vec![1, 1, 1, 1]), 6);
        assert_eq!(Solution::num_identical_pairs(vec![1, 2, 3]), 0);
    }
}
