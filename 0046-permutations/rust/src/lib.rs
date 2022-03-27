struct Solution;

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];
        Self::backtrack(&nums, 0, &mut vec![], &mut result);
        result
    }

    // 路径：记录在 path 中
    // 决策阶段：k
    // 可选列表：nums 中除掉存在与 path 中的数据
    fn backtrack(nums: &[i32], k: i32, path: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        // 结束条件
        if k == nums.len() as i32 {
            result.push(path.clone());
            return;
        }
        for num in nums {
            if path.contains(num) {
                continue;
            }
            // 做选择
            path.push(*num);
            // 递归
            Self::backtrack(nums, k + 1, path, result);
            // 撤销选择
            path.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::permute(vec![0, 1]), vec![vec![0, 1], vec![1, 0]]);
        assert_eq!(Solution::permute(vec![1]), vec![vec![1]]);
    }
}
