impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix.is_empty() || matrix[0].is_empty() {
            return false;
        }
        let (row, col) = (matrix.len(), matrix[0].len());
        let (mut left, mut right) = (0i32, (row * col - 1) as i32);
        while left <= right {
            let mid = left + (right - left) / 2;
            let (r, c) = (mid as usize / col, mid as usize % col);
            match target.cmp(&matrix[r][c]) {
                std::cmp::Ordering::Equal => return true,
                std::cmp::Ordering::Less => {
                    right = mid - 1;
                }
                std::cmp::Ordering::Greater => {
                    left = mid + 1;
                }
            }
        }
        false
    }

    pub fn search_matrix2(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let mut arr = vec![];
        for row in matrix {
            for x in row {
                arr.push(x);
            }
        }
        arr.binary_search(&target).is_ok()
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert!(!Solution::search_matrix(vec![vec![1]], 0));
        assert!(!Solution::search_matrix(vec![vec![1, 1]], 0));
        assert!(Solution::search_matrix(
            vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
            3
        ));
        assert!(!Solution::search_matrix(
            vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
            13
        ));
    }
}
