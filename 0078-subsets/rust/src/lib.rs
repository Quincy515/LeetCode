struct Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];
        Solution::backtrack(&nums, 0, &mut vec![], &mut result);
        result
    }

    // k 阶段
    // path 路径
    // nums[k] 选或不选 - 可选列表
    fn backtrack(nums: &Vec<i32>, k: i32, path: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if k == nums.len() as i32 {
            result.push(path.clone());
            return;
        }
        Self::backtrack(nums, k + 1, path, result);
        path.push(nums[k as usize]);
        Self::backtrack(nums, k + 1, path, result);
        path.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::subsets(vec![1, 2, 3]),
            vec![
                vec![],
                vec![1],
                vec![2],
                vec![1, 2],
                vec![3],
                vec![1, 3],
                vec![2, 3],
                vec![1, 2, 3]
            ]
        );
        assert_eq!(Solution::subsets(vec![0]), vec![vec![], vec![0]]);
    }
}
