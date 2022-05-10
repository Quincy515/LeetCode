use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];
        let mut hash_table = HashMap::new();
        for num in &candidates {
            if !hash_table.contains_key(num) {
                hash_table.insert(num, 1);
            } else {
                hash_table.insert(num, hash_table[num] + 1);
            }
        }
        let mut nums = vec![];
        let mut counts = vec![];
        for num in &candidates {
            if hash_table.contains_key(num) {
                nums.push(*num);
                counts.push(*hash_table.get(num).unwrap());
                hash_table.remove(num);
            }
        }
        Solution::backtrack(&mut nums, &mut counts, 0, target, &mut vec![], &mut result);
        result
    }

    fn backtrack(
        nums: &mut Vec<i32>,
        counts: &mut Vec<i32>,
        k: i32,
        left: i32,
        path: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
    ) {
        if left == 0 {
            result.push(path.clone());
            return;
        }
        if left < 0 || k == nums.len() as i32 {
            return;
        }
        let mut count = 0;
        while count <= counts[k as usize] {
            for _ in 0..count {
                path.push(nums[k as usize]);
            }
            Self::backtrack(
                nums,
                counts,
                k + 1,
                left - count * nums[k as usize],
                path,
                result,
            );
            for _ in 0..count {
                path.pop();
            }
            count += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::combination_sum2(vec![1, 1, 1, 1, 1, 1, 1], 7),
            vec![vec![1, 1, 1, 1, 1, 1, 1]]
        );
        assert_eq!(
            Solution::combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8),
            vec![vec![7, 1], vec![6, 2], vec![6, 1, 1], vec![5, 2, 1],]
        );
        assert_eq!(
            Solution::combination_sum2(vec![2, 5, 2, 1, 2], 5),
            vec![vec![5], vec![2, 2, 1],]
        );
    }
}
