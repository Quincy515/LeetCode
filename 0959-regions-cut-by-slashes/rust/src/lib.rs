struct Solution;

impl Solution {
    pub fn regions_by_slashes(grid: Vec<String>) -> i32 {
        let grid: Vec<Vec<char>> = grid.iter().map(|s| s.chars().collect::<Vec<_>>()).collect();
        let (rows, cols) = (grid.len(), grid[0].len());
        let mut uf = UnionFind::new(4 * rows * cols);
        println!("{grid:?}, {rows}, {cols}, {uf:?}");
        for i in 0..rows {
            for j in 0..cols {
                // 二维网格转换为一维表格，index 表示将单元格拆分成 4 个小三角形以后，编号为 0 的小三角行在并查集中的下标
                let index = 4 * (i * rows + j);
                let c = grid[i][j];
                println!("i: {i}, j: {j}, index: {index}, c: {c}");
                // 单元格内合并
                if c == '/' {
                    // 合并 0、3，合并 1、2
                    uf.union(index, index + 3);
                    uf.union(index + 1, index + 2);
                } else if c == '\\' {
                    // 合并 0、1，合并 2、3
                    uf.union(index, index + 1);
                    uf.union(index + 2, index + 3);
                } else {
                    uf.union(index, index + 1);
                    uf.union(index + 1, index + 2);
                    uf.union(index + 2, index + 3);
                }

                // 单元格间合并
                // 向右合并：1（当前）、3（右一列）
                if j + 1 < cols {
                    uf.union(index + 1, 4 * (i * rows + j + 1) + 3);
                }
                // 向下合并：2（当前）、0（下一列）
                if i + 1 < rows {
                    uf.union(index + 2, 4 * ((i + 1) * rows + j));
                }
            }
        }

        uf.count as i32
    }
}

#[derive(Debug)]
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    count: usize,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n + 1).collect::<Vec<_>>(),
            rank: vec![0; n + 1],
            count: n,
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if x != self.parent[x] {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let (x, y) = (self.find(x), self.find(y));
        if x == y {
            return false;
        }
        if self.rank[x] < self.rank[y] {
            self.parent[x] = y;
        } else if self.rank[x] > self.rank[y] {
            self.parent[y] = x;
        } else {
            self.parent[x] = y;
            self.rank[y] += 1;
        }
        self.count -= 1;
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // assert_eq!(
        //     Solution::regions_by_slashes(vec![" /".to_owned(), "/ ".to_owned()]),
        //     2
        // );
        // assert_eq!(
        //     Solution::regions_by_slashes(vec![" /".to_owned(), "  ".to_owned()]),
        //     1
        // );
        assert_eq!(
            Solution::regions_by_slashes(vec!["/\\".to_owned(), "\\/".to_owned()]),
            5
        );
    }
}
