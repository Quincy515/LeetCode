# [1605. 给定行和列的和求可行矩阵](https://leetcode-cn.com/problems/find-valid-matrix-given-row-and-column-sums/description/)

给你两个非负整数数组 `rowSum` 和 `colSum` ，其中 `rowSum[i]` 是二维矩阵中第 `i` 行元素的和， `colSum[j]` 是第 `j` 列元素的和。换言之你不知道矩阵里的每个元素，但是你知道每一行和每一列的和。

请找到大小为 `rowSum.length x colSum.length` 的任意 **非负整数** 矩阵，且该矩阵满足 `rowSum` 和 `colSum` 的要求。

请你返回任意一个满足题目要求的二维矩阵，题目保证存在 **至少一个** 可行矩阵。

 

**示例 1：**

```
输入：rowSum = [3,8], colSum = [4,7]
输出：[[3,0],
      [1,7]]
解释：
第 0 行：3 + 0 = 3 == rowSum[0]
第 1 行：1 + 7 = 8 == rowSum[1]
第 0 列：3 + 1 = 4 == colSum[0]
第 1 列：0 + 7 = 7 == colSum[1]
行和列的和都满足题目要求，且所有矩阵元素都是非负的。
另一个可行的矩阵为：[[1,2],
                  [3,5]]
```

**示例 2：**

```
输入：rowSum = [5,7,10], colSum = [8,6,8]
输出：[[0,5,0],
      [6,1,0],
      [2,0,8]]
```

**示例 3：**

```
输入：rowSum = [14,9], colSum = [6,9,8]
输出：[[0,9,5],
      [6,0,3]]
```

**示例 4：**

```
输入：rowSum = [1,0], colSum = [1]
输出：[[1],
      [0]]
```

**示例 5：**

```
输入：rowSum = [0], colSum = [0]
输出：[[0]]
```

 

**提示：**

- `1 <= rowSum.length, colSum.length <= 500`
- `0 <= rowSum[i], colSum[i] <= 108`
- `sum(rows) == sum(columns)`

------

[Discussion](https://leetcode-cn.com/problems/find-valid-matrix-given-row-and-column-sums/comments/) | [Solution](https://leetcode-cn.com/problems/find-valid-matrix-given-row-and-column-sums/solution/)

**思路：矩阵+贪心**

1、对于 **row_sum** 中的每个元素 `x = row_sum[i]` 都要摆放到每一行

2、遍历 **col_sum**，当 `x <= col_sum[i]`，直接就放在当前列位置，`res[i][j] = x;` 当前列剩余元素的和为 `col_sum[j] -= x`，当前行剩余元素的和为 `x = 0`

3、否则当前列位置放上列的和 `res[i][j] = col_sum[j];`，设置当前列剩余元素的和为 0，`col_sum[j] = 0;`，当前行剩余元素的和为 `x -= col_sum[j];`

**题解**

```rust
impl Solution {
    pub fn restore_matrix(row_sum: Vec<i32>, mut col_sum: Vec<i32>) -> Vec<Vec<i32>> {
        let (row, col) = (row_sum.len(), col_sum.len());
        let mut res = vec![vec![-1; col]; row];

        // 填充矩阵
        for i in 0..row {
            // 当前第 i 行元素的和
            let mut x = row_sum[i];
            for j in 0..col {
                // 当前第 i 行元素的和小于等于当前第 j 列的和
                if x <= col_sum[j] {
                    res[i][j] = x; // 放在该位置的值为当前第 i 行的和
                    col_sum[j] -= x; // 当前第 j 列剩余的和
                    x = 0; // 当前第 i 行剩余的和
                } else {
                    res[i][j] = col_sum[j]; // 放在该位置的值为当前第 j 列的和
                    x -= col_sum[j]; // 当前第 i 行剩余的和
                    col_sum[j] = 0; // 当前第 j 列剩余的和
                }
            }
        }

        res
    }
}
```

