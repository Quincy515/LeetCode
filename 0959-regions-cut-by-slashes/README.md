# [959.由斜杠划分区域](https://leetcode.cn/problems/regions-cut-by-slashes/description/)

在由 `1 x 1` 方格组成的 `n x n` 网格 `grid` 中，每个 `1 x 1` 方块由 `'/'`、`'\'` 或空格构成。这些字符会将方块划分为一些共边的区域。

给定网格 `grid` 表示为一个字符串数组，返回 *区域的数量* 。

请注意，反斜杠字符是转义的，因此 `'\'` 用 `'\\'` 表示。

 



**示例 1：**

![img](https://assets.leetcode.com/uploads/2018/12/15/1.png)

```
输入：grid = [" /","/ "]
输出：2
```

**示例 2：**

![img](https://assets.leetcode.com/uploads/2018/12/15/2.png)

```
输入：grid = [" /","  "]
输出：1
```

**示例 3：**

![img](https://assets.leetcode.com/uploads/2018/12/15/4.png)

```
输入：grid = ["/\\","\\/"]
输出：5
解释：回想一下，因为 \ 字符是转义的，所以 "/\\" 表示 /\，而 "\\/" 表示 \/。
```

 

**提示：**

- `n == grid.length == grid[i].length`
- `1 <= n <= 30`
- `grid[i][j]` 是 `'/'`、`'\'`、或 `' '`

------

[Discussion](https://leetcode.cn/problems/regions-cut-by-slashes/comments/) | [Solution](https://leetcode.cn/problems/regions-cut-by-slashes/solution/)

**思路**

学习自[官方题解](https://leetcode.cn/problems/regions-cut-by-slashes/solution/you-xie-gang-hua-fen-qu-yu-by-leetcode-67xb/)

「斜杠」、「反斜杠」把单元格拆分成的 2 个三角形的形态，在做合并的时候需要分类讨论。根据「斜杠」、「反斜杠」分割的特点，我们把一个单元格分割成逻辑上的 4 个部分。如下图所示：

![](https://pic.leetcode-cn.com/1611301988-RWjuqg-image.png)

我们须要遍历一次输入的二维网格 `grid`，在 单元格内 和 单元格间 进行合并。

单元格内：

- 如果是空格：合并 0、1、2、3；
- 如果是斜杠：合并 0、3，合并 1、2；
- 如果是反斜杠：合并 0、1，合并 2、3。

单元格间：

把每一个单元格拆分成 4 个小三角形以后，相邻的单元格须要合并，无须分类讨论。我们选择在遍历 `grid` 的每一个单元格的时候，分别「向右、向下」尝试合并。

<img src="https://pic.leetcode-cn.com/1611302894-vmBtyK-image.png" style="zoom:50%;" />

- 向右：合并 1 （当前单元格）和 3（当前单元格右边 1 列的单元格），上图中红色部分；
- 向下：合并 2 （当前单元格）和 0（当前单元格下边 1 列的单元格），上图中蓝色部分。

合并完成以后，并查集里连通分量的个数就是题目要求的区域的个数。

**题解**

```rust
struct Solution;

impl Solution {
    pub fn regions_by_slashes(grid: Vec<String>) -> i32 {
        let grid: Vec<Vec<char>> = grid.iter().map(|s| s.chars().collect::<Vec<_>>()).collect();
        let (rows, cols) = (grid.len(), grid[0].len());
        let mut uf = UnionFind::new(4 * rows * cols);
        for i in 0..rows {
            for j in 0..cols {
                // 二维网格转换为一维表格，index 表示将单元格拆分成 4 个小三角形以后，编号为 0 的小三角行在并查集中的下标
                let index = 4 * (i * rows + j);
                let c = grid[i][j];
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
```

