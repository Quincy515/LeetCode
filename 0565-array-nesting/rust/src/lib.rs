struct Solution;


/**
let mut result = vec![];
def backtrack(路径, 选择列表) {
    if 满足结束条件 {
        result.push(路径);
        return
    }

    for 选择 in 选择列表 {
        // 做选择
        将该选择从选择列表中移除
        路径.push(选择)
        backtrack(路径，选择列表);
        // 撤销选择
        路径.remove(选择)
        将该选择恢复到选择列表
    }
}
*/
impl Solution {
    pub fn array_nesting(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut result = 0;
        let mut cnt = 0;
        let mut hash = [false;200001];
        for i in 0..n {
            if !hash[i] {
                cnt = 0;
                Self::dfs(&nums, i, &mut hash, &mut cnt);
            }
            result = result.max(cnt);
        }
        result
    }

    fn dfs(nums: &[i32], u: usize, hash: &mut [bool; 200001], cnt: &mut i32) {
        if hash[u] {
            return;
        }
        *cnt += 1;
        hash[u] = true;
        Self::dfs(nums, nums[u] as usize, hash, cnt);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::array_nesting(vec![5, 4, 0, 3, 1, 6, 2]), 4);
    }
}
