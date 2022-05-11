struct Solution;

impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        let (row, col) = (grid.len(), grid.first().map_or(0, Vec::len));
        let mut result = 0;
        for i in 0..row {
            for j in 0..col {
                result += if grid[i][j] < 0 { 1 } else { 0 };
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
            Solution::count_negatives(vec![
                vec![4, 3, 2, -1],
                vec![3, 2, 1, -1],
                vec![1, 1, -1, -2],
                vec![-1, -1, -2, -3]
            ]),
            8
        );
        assert_eq!(Solution::count_negatives(vec![vec![3, 2], vec![1, 0]]), 0);
    }
}
