# [130.被围绕的区域](https://leetcode.cn/problems/surrounded-regions/description/)

给你一个 `m x n` 的矩阵 `board` ，由若干字符 `'X'` 和 `'O'` ，找到所有被 `'X'` 围绕的区域，并将这些区域里所有的 `'O'` 用 `'X'` 填充。

 

**示例 1：**

![img](https://assets.leetcode.com/uploads/2021/02/19/xogrid.jpg)

```
输入：board = [["X","X","X","X"],["X","O","O","X"],["X","X","O","X"],["X","O","X","X"]]
输出：[["X","X","X","X"],["X","X","X","X"],["X","X","X","X"],["X","O","X","X"]]
解释：被围绕的区间不会存在于边界上，换句话说，任何边界上的 'O' 都不会被填充为 'X'。 任何不在边界上，或不与边界上的 'O' 相连的 'O' 最终都会被填充为 'X'。如果两个元素在水平或垂直方向相邻，则称它们是“相连”的。
```

**示例 2：**

```
输入：board = [["X"]]
输出：[["X"]]
```

 

**提示：**

- `m == board.length`
- `n == board[i].length`
- `1 <= m, n <= 200`
- `board[i][j]` 为 `'X'` 或 `'O'`

------

[Discussion](https://leetcode.cn/problems/surrounded-regions/comments/) | [Solution](https://leetcode.cn/problems/surrounded-regions/solution/)

### 解题思路
[labuladong 的算法小抄 > 第一章、手把手刷数据结构 > 手把手刷图算法 > 并查集（Union-Find）](https://labuladong.github.io/algo/2/20/50/)

可以把那些不需要被替换的 O 看成一个独立的集合，它们有一个共同的根节点叫 dummy，这些 O 和 dummy 互相连通，而那些需要被替换的 O 与 dummy 不连通。

**知识点**：
- 二维坐标映射到一维 (x, y) => x * n + y;
- 方向数组 d 是上下左右搜索的常用方法：`let d = [[1, 0], [0, 1], [0, -1], [-1, 0]]`
- 并查集

**总结**：
> 主要思路是适时增加虚拟节点，想办法让元素「分门别类」，建立动态连通关系。

### 代码

```rust
impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        if board.is_empty() { return; }
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
        for i in 1..m-1 {
            for j in 1..n-1 {
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
        for i in 1..m-1 {
            for j in 1..n-1 {
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
            parent: (0..n + 1).collect::<Vec<_>>()
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
        if x_root == y_root { return false; }

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
```