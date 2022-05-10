use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];
        let mut hm = HashMap::new();
        for num in &nums {
            let mut count = 1;
            if hm.contains_key(num) {
                count += hm.get(num).unwrap();
            }
            hm.insert(num, count);
        }
        let n = hm.len();
        let mut unique_nums = vec![0; n];
        let mut counts = vec![0; n];
        let mut k = 0;
        for num in &nums {
            if hm.contains_key(num) {
                unique_nums[k] = *num;
                counts[k] = *hm.get(num).unwrap();
                k += 1;
                hm.remove(num);
            }
        }
        Solution::backtrack(
            &mut unique_nums,
            &mut counts,
            0,
            &mut vec![],
            nums.len(),
            &mut result,
        );
        result
    }

    fn backtrack(
        unique_nums: &mut Vec<i32>,
        counts: &mut Vec<i32>,
        k: i32,
        path: &mut Vec<i32>,
        n: usize,
        result: &mut Vec<Vec<i32>>,
    ) {
        if k == n as i32 {
            result.push(path.clone());
            return;
        }
        for i in 0..unique_nums.len() {
            if counts[i] == 0 {
                continue;
            }
            path.push(unique_nums[i]); // 添加选择
            counts[i] -= 1;
            Self::backtrack(unique_nums, counts, k + 1, path, n, result);
            path.pop(); // 撤销选择
            counts[i] += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::permute_unique(vec![1, 1, 2]),
            vec![vec![2, 1, 1], vec![1, 2, 1], vec![1, 1, 2],]
        );
        assert_eq!(
            Solution::permute_unique(vec![1, 1, 1]),
            vec![vec![1, 1, 1],]
        );
        assert_eq!(
            Solution::permute_unique(vec![1, 1, 1, 2]),
            vec![
                vec![2, 1, 1, 1],
                vec![1, 2, 1, 1],
                vec![1, 1, 2, 1],
                vec![1, 1, 1, 2],
            ]
        );
    }
}
