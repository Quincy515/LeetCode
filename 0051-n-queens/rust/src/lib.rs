impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut result: Vec<Vec<String>> = vec![];
        let mut board: Vec<Vec<char>> = vec![vec!['.'; n as usize]; n as usize];
        Self::backtrack(0, n as usize, &mut board, &mut result);
        result
    }

    // row: 阶段
    // board: 路径，记录已经做出的决策
    // 可选列表：通过 board 推导出来，没有显示记录
    fn backtrack(row: usize, n: usize, board: &mut [Vec<char>], result: &mut Vec<Vec<String>>) {
        // 结束条件，得到可行解
        if row == n {
            let mut snapshot = vec![];
            for row in board {
                let mut line = String::new();
                for c in row {
                    line.push(*c);
                }
                snapshot.push(line);
            }
            result.push(snapshot);
            // result.push(board.iter().map(|vec| vec.iter().collect()).collect());
            return;
        }
        for col in 0..n {
            // 每一行都有 n 种放法
            if Self::is_ok(board, n, row, col) {
                // 可选列表
                board[row][col] = 'Q'; // 做选择，第 row 行的棋子放在 col 列
                Self::backtrack(row + 1, n, board, result); // 考察下一行
                board[row][col] = '.'; // 恢复选择
            }
        }
    }

    // 判断 row 行 column 列放置是否合适
    fn is_ok(board: &[Vec<char>], n: usize, row: usize, col: usize) -> bool {
        // 检查是否有冲突
        for i in 0..n {
            if board[i][col] == 'Q' {
                return false;
            }
        }
        // 检查右上对角线是否有冲突
        let (mut i, mut j) = (row as i32 - 1, col as i32 + 1);
        while i >= 0 && j < n as i32 {
            if board[i as usize][j as usize] == 'Q' {
                return false;
            }
            i -= 1;
            j += 1;
        }
        // 检查左上对角线是否有冲突
        let (mut i, mut j) = (row as i32 - 1, col as i32 - 1);
        while i >= 0 && j >= 0 {
            if board[i as usize][j as usize] == 'Q' {
                return false;
            }
            i -= 1;
            j -= 1;
        }
        true
    }
}
struct Solution;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_works() {
        assert_eq!(
            Solution::solve_n_queens(4),
            vec![
                vec![".Q..", "...Q", "Q...", "..Q."],
                vec!["..Q.", "Q...", "...Q", ".Q.."]
            ]
        );
        assert_eq!(Solution::solve_n_queens(8).len(), 92);
    }
}
