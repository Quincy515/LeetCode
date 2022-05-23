struct Solution;

impl Solution {
    pub fn max_sum_submatrix(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let (row, col) = (matrix.len(), matrix.first().map_or(0, Vec::len));
        let mut result = std::i32::MIN;

        // 枚举上边界
        for i in 0..row {
            let mut sum = vec![0; col];
            for j in i..row {
                for c in 0..col {
                    sum[c] += matrix[j][c];
                }

                let mut s = 0;
                let mut bts = std::collections::BTreeSet::new();
                bts.insert(0);
                for v in sum.iter() {
                    s += v;
                    if let Some(x) = bts.range(s - k..).next() {
                        result = result.max(s - x);
                    };
                    bts.insert(s);
                }
            }
        }
        result
    }
    pub fn max_sum_submatrix_iterator(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let (row, col) = (matrix.len(), matrix.first().map_or(0, Vec::len));
        let mut result = std::i32::MIN;

        // 枚举上边界
        for i in 0..row {
            let mut row_sum = vec![0; col];

            // 枚举下边界
            for r in matrix.iter().skip(i) {
                // 更新每列的元素和
                row_sum.iter_mut().zip(r).for_each(|(sum, val)| *sum += val);

                let mut sum = 0;
                let mut bts = std::collections::BTreeSet::new();
                bts.insert(0);
                for val in &row_sum {
                    sum += val;
                    if let Some(x) = bts.range(sum - k..).next() {
                        result = result.max(sum - x);
                        if result == k {
                            return k;
                        }
                    };
                    bts.insert(sum);
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::max_sum_submatrix(vec![vec![1, 0, 1], vec![0, -2, 3]], 2),
            2
        );
        // assert_eq!(Solution::max_sum_submatrix(vec![vec![2, 2, -1]], 3), 3);
    }
}
