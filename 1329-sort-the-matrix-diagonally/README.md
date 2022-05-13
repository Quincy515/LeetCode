# [1329.将矩阵按对角线排序](https://leetcode-cn.com/problems/sort-the-matrix-diagonally/description/)

**矩阵对角线** 是一条从矩阵最上面行或者最左侧列中的某个元素开始的对角线，沿右下方向一直到矩阵末尾的元素。例如，矩阵 `mat` 有 `6` 行 `3` 列，从 `mat[2][0]` 开始的 **矩阵对角线** 将会经过 `mat[2][0]`、`mat[3][1]` 和 `mat[4][2]` 。

给你一个 `m * n` 的整数矩阵 `mat` ，请你将同一条 **矩阵对角线** 上的元素按升序排序后，返回排好序的矩阵。

 

**示例 1：**

![img](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2020/01/25/1482_example_1_2.png)

```
输入：mat = [[3,3,1,1],[2,2,1,2],[1,1,1,2]]
输出：[[1,1,1,1],[1,2,2,2],[1,2,3,3]]
```

**示例 2：**

```
输入：mat = [[11,25,66,1,69,7],[23,55,17,45,15,52],[75,31,36,44,58,8],[22,27,33,25,68,4],[84,28,14,11,5,50]]
输出：[[5,17,4,1,52,7],[11,11,25,45,8,69],[14,23,25,44,58,15],[22,27,31,36,50,66],[84,28,75,33,55,68]]
```

 

**提示：**

- `m == mat.length`
- `n == mat[i].length`
- `1 <= m, n <= 100`
- `1 <= mat[i][j] <= 100`

------

[Discussion](https://leetcode-cn.com/problems/sort-the-matrix-diagonally/comments/) | [Solution](https://leetcode-cn.com/problems/sort-the-matrix-diagonally/solution/)

**思路**

1、枚举每行开头、每列开头的对角线元素

2、往右下角进行遍历塞入另一个数组中，然后对数组排序，再按照同样方式放置回来

**题解**

```rust
impl Solution {
    pub fn diagonal_sort(mut mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (row, col) = (mat.len(), mat.first().map_or(0, Vec::len));
        for i in 0..row {
            Self::sort(&mut mat, row, col, i, 0);
        }
        for i in 1..col {
            Self::sort(&mut mat, row, col, 0, i)
        }
        mat
    }

    fn sort(mat: &mut Vec<Vec<i32>>, row: usize, col: usize, row_start: usize, col_start: usize) {
        let mut temp = vec![];
        let (mut r, mut c) = (row_start, col_start);
        while r < row && c < col {
            temp.push(mat[r][c]);
            r += 1;
            c += 1;
        }
        temp.sort_unstable();
        let (mut r, mut c, mut x) = (row_start, col_start, 0);
        while r < row && c < col {
            mat[r][c] = temp[x];
            x += 1;
            r += 1;
            c += 1;
        }
    }
}
```

