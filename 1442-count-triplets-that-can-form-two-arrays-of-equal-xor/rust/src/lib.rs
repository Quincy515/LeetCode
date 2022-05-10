struct Solution;

impl Solution {
    pub fn count_triplets(mut arr: Vec<i32>) -> i32 {
        let mut hash = std::collections::HashMap::new();
        let mut result = 0;
        for i in 1..arr.len() {
            arr[i] ^= arr[i - 1];
        }
        for j in 1..arr.len() {
            hash.clear();
            for i in 0..j {
                let mut pre = 0;
                if i != 0 {
                    pre = arr[i - 1];
                }
                *hash.entry(arr[j - 1] ^ pre).or_insert(0) += 1;
            }
            for k in j..arr.len() {
                result += *hash.entry(arr[k] ^ arr[j - 1]).or_insert(0);
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
        assert_eq!(Solution::count_triplets(vec![2, 3, 1, 6, 7]), 4);
        assert_eq!(Solution::count_triplets(vec![1, 1, 1, 1, 1]), 10);
        assert_eq!(Solution::count_triplets(vec![2, 3]), 0);
        assert_eq!(Solution::count_triplets(vec![1, 3, 5, 7, 9]), 3);
        assert_eq!(
            Solution::count_triplets(vec![7, 11, 12, 9, 5, 2, 7, 17, 22]),
            8
        );
    }
}
