use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
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
        Self::backtrack(&mut unique_nums, &mut counts, 0, &mut vec![], &mut result);
        result
    }

    fn backtrack(
        unique_nums: &mut Vec<i32>,
        counts: &mut Vec<i32>,
        k: i32,
        path: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
    ) {
        if k == unique_nums.len() as i32 {
            result.push(path.clone());
            return;
        }
        let mut count = 0;
        while count <= counts[k as usize] {
            for _ in 0..count {
                path.push(unique_nums[k as usize]);
            }
            Self::backtrack(unique_nums, counts, k + 1, path, result);
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
        assert_eq!(Solution::subsets_with_dup(vec![0]), vec![vec![], vec![0]]);
    }
}
