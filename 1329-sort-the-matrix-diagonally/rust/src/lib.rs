struct Solution;

impl Solution {
    pub fn diagonal_sort(mut mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (row, col) = (mat.len(), mat.first().map_or(0, Vec::len));
        for i in 0..row {
            Self::sort(&mut mat, row, col, i, 0);
        }
        for i in 1..col {
            Self::sort(&mut mat, row, col, 0, i)
        }
        mat
    }

    fn sort(mat: &mut Vec<Vec<i32>>, row: usize, col: usize, row_start: usize, col_start: usize) {
        let mut temp = vec![];
        let (mut r, mut c) = (row_start, col_start);
        while r < row && c < col {
            temp.push(mat[r][c]);
            r += 1;
            c += 1;
        }
        temp.sort_unstable();
        let (mut r, mut c, mut x) = (row_start, col_start, 0);
        while r < row && c < col {
            mat[r][c] = temp[x];
            x += 1;
            r += 1;
            c += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::diagonal_sort(vec![vec![3, 3, 1, 1], vec![2, 2, 1, 2], vec![1, 1, 1, 2]]),
            vec![vec![1, 1, 1, 1], vec![1, 2, 2, 2], vec![1, 2, 3, 3]]
        );
        assert_eq!(
            Solution::diagonal_sort(vec![
                vec![11, 25, 66, 1, 69, 7],
                vec![23, 55, 17, 45, 15, 52],
                vec![75, 31, 36, 44, 58, 8],
                vec![22, 27, 33, 25, 68, 4],
                vec![84, 28, 14, 11, 5, 50]
            ]),
            vec![
                vec![5, 17, 4, 1, 52, 7],
                vec![11, 11, 25, 45, 8, 69],
                vec![14, 23, 25, 44, 58, 15],
                vec![22, 27, 31, 36, 50, 66],
                vec![84, 28, 75, 33, 55, 68]
            ]
        );
    }
}
