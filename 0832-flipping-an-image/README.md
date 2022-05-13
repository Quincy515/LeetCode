# [832.翻转图像](https://leetcode-cn.com/problems/flipping-an-image/description/)

给定一个 `n x n` 的二进制矩阵 `image` ，先 **水平** 翻转图像，然后 **反转** 图像并返回 *结果* 。

**水平**翻转图片就是将图片的每一行都进行翻转，即逆序。

- 例如，水平翻转 `[1,1,0]` 的结果是 `[0,1,1]`。

**反转**图片的意思是图片中的 `0` 全部被 `1` 替换， `1` 全部被 `0` 替换。

- 例如，反转 `[0,1,1]` 的结果是 `[1,0,0]`。

 

**示例 1：**

```
输入：image = [[1,1,0],[1,0,1],[0,0,0]]
输出：[[1,0,0],[0,1,0],[1,1,1]]
解释：首先翻转每一行: [[0,1,1],[1,0,1],[0,0,0]]；
     然后反转图片: [[1,0,0],[0,1,0],[1,1,1]]
```

**示例 2：**

```
输入：image = [[1,1,0,0],[1,0,0,1],[0,1,1,1],[1,0,1,0]]
输出：[[1,1,0,0],[0,1,1,0],[0,0,0,1],[1,0,1,0]]
解释：首先翻转每一行: [[0,0,1,1],[1,0,0,1],[1,1,1,0],[0,1,0,1]]；
     然后反转图片: [[1,1,0,0],[0,1,1,0],[0,0,0,1],[1,0,1,0]]
```

 

**提示：**



- `n == image.length`
- `n == image[i].length`
- `1 <= n <= 20`
- `images[i][j]` == `0` 或 `1`.

------

[Discussion](https://leetcode-cn.com/problems/flipping-an-image/comments/) | [Solution](https://leetcode-cn.com/problems/flipping-an-image/solution/)

**思路**

1、枚举矩阵的每一行，对每一行进行一个翻转，翻转操作类似于字符串的反转

2、然后对矩阵的每个数字异或 1 即可，实现一个原地反转

**题解**

```rust
impl Solution {
    pub fn flip_and_invert_image(mut image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (row, col) = (image.len(), image.first().map_or(0, Vec::len));
        for i in 0..row {
            for j in 0..col / 2 {
                image[i].swap(j, col - 1 - j);
            }
            for j in 0..col {
                image[i][j] ^= 1;
            }
        }

        image
    }
}
```

