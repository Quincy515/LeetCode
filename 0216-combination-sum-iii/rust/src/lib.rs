struct Solution;

impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        // 1 ~ 9，选 k 个数，和为 n
        let mut result = vec![];
        Solution::backtrack(k, n, 1, 0, &mut vec![], &mut result);

        result
    }

    fn backtrack(
        k: i32,
        n: i32,
        step: i32,
        sum: i32,
        path: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
    ) {
        if sum == n && path.len() as i32 == k {
            result.push(path.clone());
            return;
        }
        if sum > n || path.len() as i32 > k || step > 9 {
            return;
        }
        Self::backtrack(k, n, step + 1, sum, path, result);
        path.push(step);
        Self::backtrack(k, n, step + 1, sum + step, path, result);
        path.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::combination_sum3(3, 9),
            vec![vec![1, 2, 6], vec![1, 3, 5], vec![2, 3, 4]]
        );
    }
}
