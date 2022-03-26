impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];

        Self::backtrack(n, k, 1, &mut vec![], &mut result);

        result
    }

    // n, k 必须的参数
    // step 阶段
    // path 路径
    // step 选与不选 - 可选列表
    fn backtrack(n: i32, k: i32, step: i32, path: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if path.len() == k as usize {
            result.push(path.clone());
            return;
        }
        if step == n + 1 {
            return;
        }
        Self::backtrack(n, k, step + 1, path, result);
        path.push(step);
        Self::backtrack(n, k, step + 1, path, result);
        path.pop();
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::combine(1, 1), vec![vec![1],]);
    }
}
