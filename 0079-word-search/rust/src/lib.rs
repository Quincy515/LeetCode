struct Solution;

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let (h, w) = (board.len(), board.first().map_or(0, Vec::len));
        let word = word.chars().collect::<Vec<_>>();
        for i in 0..h {
            for j in 0..w {
                let mut visited = vec![vec![false; w]; h];
                if Solution::dfs(&board, &word, i, j, 0, &mut visited) {
                    return true;
                }
            }
        }
        false
    }

    fn dfs(
        board: &[Vec<char>],
        word: &[char],
        i: usize,
        j: usize,
        k: usize,
        visited: &mut Vec<Vec<bool>>,
    ) -> bool {
        if i >= board.len() || j >= board.first().map_or(0, Vec::len) || visited[i][j] {
            return false;
        }
        if word[k] != board[i][j] {
            return false;
        }
        if k == word.len() - 1 {
            return true;
        }
        visited[i][j] = true;

        if (i >= 1 && Solution::dfs(board, word, i - 1, j, k + 1, visited))
            || Solution::dfs(board, word, i + 1, j, k + 1, visited)
            || (j >= 1 && Solution::dfs(board, word, i, j - 1, k + 1, visited))
            || Solution::dfs(board, word, i, j + 1, k + 1, visited)
        {
            return true;
        }
        visited[i][j] = false;
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_79() {
        assert!(!Solution::exist(
            vec![
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'C', 'S'],
                vec!['A', 'D', 'E', 'E'],
            ],
            "ABCB".to_owned()
        ));
        assert!(Solution::exist(vec![vec!['a']], "a".to_owned()));
        assert!(Solution::exist(
            vec![
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'C', 'S'],
                vec!['A', 'D', 'E', 'E'],
            ],
            "ABCCED".to_owned()
        ));
        assert!(Solution::exist(
            vec![
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'C', 'S'],
                vec!['A', 'D', 'E', 'E'],
            ],
            "SEE".to_owned()
        ));
    }
}
