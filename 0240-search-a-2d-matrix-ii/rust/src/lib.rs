struct Solution;
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let (m, n) = (matrix.len(), matrix[0].len());
        let (mut i, mut j) = (0, n - 1);
        while i < m && (j as i32) >= 0 {
            if target < matrix[i][j] {
                if j == 0 {
                    return false;
                }
                j -= 1;
            } else if target > matrix[i][j] {
                i += 1;
            } else {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let matrix = vec![
            vec![1, 4, 7, 11, 15],
            vec![2, 5, 8, 12, 19],
            vec![3, 6, 9, 16, 22],
            vec![10, 13, 14, 17, 24],
            vec![18, 21, 23, 26, 30],
        ];
        assert_eq!(true, Solution::search_matrix(matrix, 5));
    }

    #[test]
    fn it_works2() {
        let matrix = vec![vec![-5]];
        assert_eq!(false, Solution::search_matrix(matrix, -10));
    }
}
