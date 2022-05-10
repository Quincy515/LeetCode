struct Solution;

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let (h, w) = (grid.len(), grid.first().map_or(0, Vec::len));
        let mut visited: Vec<Vec<bool>> = vec![vec![false; w]; h];

        let mut result: i32 = 0;
        for i in 0..h {
            for j in 0..w {
                if !visited[i][j] && grid[i][j] == '1' {
                    result += 1;
                    Solution::dfs(&grid, i, j, h, w, &mut visited);
                }
            }
        }
        result
    }

    fn dfs(
        grid: &[Vec<char>],
        i: usize,
        j: usize,
        h: usize,
        w: usize,
        visited: &mut Vec<Vec<bool>>,
    ) {
        let directions: Vec<Vec<i32>> = vec![vec![-1, 0], vec![1, 0], vec![0, -1], vec![0, 1]];
        visited[i][j] = true;
        for k in 0..4 {
            let newi = i as i32 + directions[k][0];
            let newj = j as i32 + directions[k][1];
            if newi >= 0
                && newi < h as i32
                && newj >= 0
                && newj < w as i32
                && !visited[newi as usize][newj as usize]
                && grid[newi as usize][newj as usize] == '1'
            {
                Solution::dfs(grid, newi as usize, newj as usize, h, w, visited);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::num_islands(vec![
                vec!['1', '1', '1', '1', '0',],
                vec!['1', '1', '0', '1', '0',],
                vec!['1', '1', '0', '0', '0',],
                vec!['0', '0', '0', '0', '0',],
            ]),
            1
        );
        assert_eq!(
            Solution::num_islands(vec![
                vec!['1', '1', 'o', '1', '0',],
                vec!['1', '1', '0', '1', '0',],
                vec!['1', '1', '0', '0', '0',],
                vec!['0', '0', '0', '1', '1',],
            ]),
            3
        );
    }
}
