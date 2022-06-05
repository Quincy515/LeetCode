impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        if board.is_empty() {
            return;
        }
        // *** m 行，n 列，二维坐标 (x, y) 转换为一维 x * n + y
        let (m, n) = (board.len(), board.first().map_or(0, Vec::len));
        // 给 dummy 留一个额外的位置
        let mut uf = UnionFind::new(m * n + 1);
        let dummy = m * n;
        // 将首列和末列的 0 与 dummy 连通
        for i in 0..m {
            if board[i][0] == 'O' {
                uf.union(i * n, dummy);
            }
            if board[i][n - 1] == 'O' {
                uf.union(i * n + n - 1, dummy);
            }
        }
        // 将首行和末行的 0 与 dummy 连通
        for j in 0..n {
            if board[0][j] == 'O' {
                uf.union(j, dummy);
            }
            if board[m - 1][j] == 'O' {
                uf.union(n * (m - 1) + j, dummy);
            }
        }
        // 方向数组 d 是上下左右搜索的常用方法
        let d = [[1, 0], [0, 1], [0, -1], [-1, 0]];
        for i in 1..m - 1 {
            for j in 1..n - 1 {
                if board[i][j] == 'O' {
                    // 将此 O 与上下左右的 O 连通
                    for k in 0..4 {
                        let x = (i as i32 + d[k][0]) as usize;
                        let y = (j as i32 + d[k][1]) as usize;
                        if board[x][y] == 'O' {
                            uf.union(x * n + y, i * n + j);
                        }
                    }
                }
            }
        }
        // 所有不和 dummy 连通的 O，都要被替换
        for i in 1..m - 1 {
            for j in 1..n - 1 {
                if !uf.connect(dummy, i * n + j) {
                    board[i][j] = 'X';
                }
            }
        }
    }
}

// 并查集模版
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn connect(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    fn new(n: usize) -> Self {
        Self {
            rank: vec![0; n + 1],
            parent: (0..n + 1).collect::<Vec<_>>(),
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if x != self.parent[x] {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let (x_root, y_root) = (self.find(x), self.find(y));
        if x_root == y_root {
            return false;
        }

        if self.rank[x_root] > self.rank[y_root] {
            self.parent[y_root] = x_root;
        } else if self.rank[x_root] < self.rank[y_root] {
            self.parent[x_root] = y_root;
        } else {
            self.parent[x_root] = y_root;
            self.rank[y_root] += 1;
        }

        true
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut vec = vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'O', 'X'],
            vec!['X', 'X', 'O', 'X'],
            vec!['X', 'O', 'X', 'X'],
        ];
        Solution::solve(&mut vec);
        assert_eq!(
            vec,
            vec![
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'O', 'X', 'X']
            ]
        );
    }
}
