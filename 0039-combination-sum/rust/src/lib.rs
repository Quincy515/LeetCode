struct Solution;

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];
        Self::backtrack(&candidates, 0, target, &mut vec![], &mut result);

        result
    }

    fn backtrack(
        candidates: &Vec<i32>,
        k: i32,
        left: i32,
        path: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
    ) {
        if left == 0 {
            result.push(path.clone());
            return;
        }
        if k == candidates.len() as i32 {
            return;
        }
        for i in 0..=left / candidates[k as usize] {
            for _ in 0..i {
                path.push(candidates[k as usize]);
            }
            Self::backtrack(
                candidates,
                k + 1,
                left - candidates[k as usize] * i,
                path,
                result,
            );
            for _ in 0..i {
                path.pop();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::combination_sum(vec![1], 7),
            vec![vec![1, 1, 1, 1, 1, 1, 1]]
        );
        assert_eq!(
            Solution::combination_sum(vec![2, 3, 6, 7], 7),
            vec![vec![7], vec![3, 2, 2],]
        );
        assert_eq!(
            Solution::combination_sum(vec![2, 3, 5], 8),
            vec![vec![5, 3], vec![3, 3, 2], vec![2, 2, 2, 2],]
        );
    }
}
