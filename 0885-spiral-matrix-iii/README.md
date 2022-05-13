# [885.螺旋矩阵 III](https://leetcode-cn.com/problems/spiral-matrix-iii/description/)

在 `R` 行 `C` 列的矩阵上，我们从 `(r0, c0)` 面朝东面开始

这里，网格的西北角位于第一行第一列，网格的东南角位于最后一行最后一列。

现在，我们以顺时针按螺旋状行走，访问此网格中的每个位置。

每当我们移动到网格的边界之外时，我们会继续在网格之外行走（但稍后可能会返回到网格边界）。

最终，我们到过网格的所有 `R * C` 个空间。

按照访问顺序返回表示网格位置的坐标列表。

 

**示例 1：**

```
输入：R = 1, C = 4, r0 = 0, c0 = 0
输出：[[0,0],[0,1],[0,2],[0,3]]
```

 

**示例 2：**

```
输入：R = 5, C = 6, r0 = 1, c0 = 4
输出：[[1,4],[1,5],[2,5],[2,4],[2,3],[1,3],[0,3],[0,4],[0,5],[3,5],[3,4],[3,3],[3,2],[2,2],[1,2],[0,2],[4,5],[4,4],[4,3],[4,2],[4,1],[3,1],[2,1],[1,1],[0,1],[4,0],[3,0],[2,0],[1,0],[0,0]]
```

 

**提示：**

1. `1 <= R <= 100`
2. `1 <= C <= 100`
3. `0 <= r0 < R`
4. `0 <= c0 < C`

------

[Discussion](https://leetcode-cn.com/problems/spiral-matrix-iii/comments/) | [Solution](https://leetcode-cn.com/problems/spiral-matrix-iii/solution/)

**思路**

螺旋矩阵

1. 确立四个边界Left, Right, Upper, Bottom;
2. 当一个方向达到边界时，调整方向；
3. 根据方向更新下一个节点
4. 当节点在矩阵范围内，则加入到res中

**题解**

```rust
impl Solution {
    pub fn spiral_matrix_iii(rows: i32, cols: i32, r_start: i32, c_start: i32) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        //顺时针方向
        let direction = [[0, 1], [1, 0], [0, -1], [-1, 0]];
        //{x, y}为当前位置，num为当前查找的数字，dir为当前的方向
        let (mut x, mut y, mut num, mut dir) = (r_start, c_start, 1, 0);
        //四个方向的边界
        let (mut left, mut right, mut upper, mut bottom) =
            (c_start - 1, c_start + 1, r_start - 1, r_start + 1);
        while num <= rows * cols {
            //{x, y} 位置在矩阵中
            if x >= 0 && x < rows && y >= 0 && y < cols {
                res.push(vec![x, y]);
                num += 1;
            }
            //向右到右边界
            if dir == 0 && y == right {
                dir += 1; //调转方向向下
                right += 1; //右边界右移
            }
            //向下到底边界
            else if dir == 1 && x == bottom {
                dir += 1;
                bottom += 1; //底边界下移
            }
            //向左到左边界
            else if dir == 2 && y == left {
                dir += 1;
                left -= 1; // 左边界左移
            }
            // 向上到上边界
            else if dir == 3 && x == upper {
                dir = 0;
                upper -= 1; //上边界上移
            }
            x += direction[dir][0]; // 下一个节点
            y += direction[dir][1];
        }

        res
    }
}
```

