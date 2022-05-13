struct Solution;

impl Solution {
    pub fn restore_matrix(row_sum: Vec<i32>, mut col_sum: Vec<i32>) -> Vec<Vec<i32>> {
        let (row, col) = (row_sum.len(), col_sum.len());
        let mut res = vec![vec![-1; col]; row];
        for i in 0..row {
            let mut x = row_sum[i];
            for j in 0..col {
                if x <= col_sum[j] {
                    res[i][j] = x;
                    col_sum[j] -= x;
                    x -= x;
                } else {
                    res[i][j] = col_sum[j];
                    x -= col_sum[j];
                    col_sum[j] = 0;
                }
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::restore_matrix(vec![3, 8], vec![4, 7]),
            vec![vec![3, 0], vec![1, 7]]
        );
        assert_eq!(
            Solution::restore_matrix(vec![5, 7, 10], vec![8, 6, 8]),
            vec![vec![0, 5, 0], vec![6, 1, 0], vec![2, 0, 8]]
        );
        assert_eq!(
            Solution::restore_matrix(vec![14, 9], vec![6, 9, 8]),
            vec![vec![0, 9, 5], vec![6, 0, 3]]
        );
        assert_eq!(
            Solution::restore_matrix(vec![1, 0], vec![1]),
            vec![vec![1], vec![0]]
        );
        assert_eq!(Solution::restore_matrix(vec![0], vec![0]), vec![vec![0]]);
    }
}
