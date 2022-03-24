impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut rows = vec![vec![false; 10]; 9];
        let mut cols = vec![vec![false; 10]; 9];
        let mut blocks = vec![vec![vec![false; 10]; 3]; 3];
        let mut solved = false;
        for i in 0..9 {
            for j in 0..9 {
                if board[i][j] != '.' {
                    let num = (board[i][j] as u8 - b'0') as usize;
                    // let num = board[i][j].to_digit(10).unwrap() as usize -1;
                    rows[i][num] = true;
                    cols[j][num] = true;
                    blocks[i / 3][j / 3][num] = true;
                }
            }
        }
        Self::backtrack(0, 0, board, &mut solved, &mut rows, &mut cols, &mut blocks);
    }

    fn backtrack(
        row: i32,
        col: i32,
        board: &mut Vec<Vec<char>>,
        solved: &mut bool,
        rows: &mut Vec<Vec<bool>>,
        cols: &mut Vec<Vec<bool>>,
        blocks: &mut Vec<Vec<Vec<bool>>>,
    ) {
        if row == 9 {
            *solved = true;
            return;
        }
        if board[row as usize][col as usize] != '.' {
            let mut next_row = row;
            let mut next_col = col + 1;
            if col == 8 {
                next_row = row + 1;
                next_col = 0;
            }
            Self::backtrack(next_row, next_col, board, solved, rows, cols, blocks);
            if *solved {
                return;
            }
        } else {
            for num in 1..=9 {
                if !rows[row as usize][num as usize]
                    && !cols[col as usize][num as usize]
                    && !blocks[row as usize / 3][col as usize / 3][num as usize]
                {
                    // board[row as usize][col as usize] = (num + b'0') as char; // 数字转换为字符
                    board[row as usize][col as usize] = std::char::from_digit(num, 10).unwrap(); // 数字转换为字符
                    rows[row as usize][num as usize] = true;
                    cols[col as usize][num as usize] = true;
                    blocks[row as usize / 3][col as usize / 3][num as usize] = true;
                    let mut next_row = row;
                    let mut next_col = col + 1;
                    if col == 8 {
                        next_row = row + 1;
                        next_col = 0;
                    }
                    Self::backtrack(next_row, next_col, board, solved, rows, cols, blocks);
                    if *solved {
                        return;
                    }
                    board[row as usize][col as usize] = '.';
                    rows[row as usize][num as usize] = false;
                    cols[row as usize][num as usize] = false;
                    blocks[row as usize / 3][col as usize / 3][num as usize] = false;
                }
            }
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        Solution::solve_sudoku(&mut board);
        let res = vec![
            vec!['5', '3', '4', '6', '7', '8', '9', '1', '2'],
            vec!['6', '7', '2', '1', '9', '5', '3', '4', '8'],
            vec!['1', '9', '8', '3', '4', '2', '5', '6', '7'],
            vec!['8', '5', '9', '7', '6', '1', '4', '2', '3'],
            vec!['4', '2', '6', '8', '5', '3', '7', '9', '1'],
            vec!['7', '1', '3', '9', '2', '4', '8', '5', '6'],
            vec!['9', '6', '1', '5', '3', '7', '2', '8', '4'],
            vec!['2', '8', '7', '4', '1', '9', '6', '3', '5'],
            vec!['3', '4', '5', '2', '8', '6', '1', '7', '9'],
        ];
        assert_eq!(board, res);
    }
}
