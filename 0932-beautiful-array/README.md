# [932.漂亮数组](https://leetcode.cn/problems/beautiful-array/description/)

对于某些固定的 `N`，如果数组 `A` 是整数 `1, 2, ..., N` 组成的排列，使得：

对于每个 `i < j`，都**不存在** `k` 满足 `i < k < j` 使得 `A[k] * 2 = A[i] + A[j]`。

那么数组 `A` 是漂亮数组。

 

给定 `N`，返回**任意**漂亮数组 `A`（保证存在一个）。

 

**示例 1：**

```
输入：4
输出：[2,1,4,3]
```

**示例 2：**

```
输入：5
输出：[3,1,2,5,4]
```

 

**提示：**

- `1 <= N <= 1000`

 

------

[Discussion](https://leetcode.cn/problems/beautiful-array/comments/) | [Solution](https://leetcode.cn/problems/beautiful-array/solution/)

**思路**

`A[k] * 2 ！= A[i] + A[j]`，说明等式左边是偶数，那么右边为奇数可以保证永不相等。
可以将A[i]映射为奇数，A[j]映射为偶数。问题转化为，左区间映射为奇数，右区间映射为偶数，
由于漂亮数组经过 `k*a+b` 的变化后，仍然是漂亮数组。
因此，可以分解成若干子问题，用分治求解。

**题解**

```rust
impl Solution {
    pub fn beautiful_array(n: i32) -> Vec<i32> {
        let mut result = vec![];
        if n == 1 {
            result.push(1);
            return result;
        }
        let odd_num = (n + 1) / 2;
        let even_num = n / 2;
        let (left_vec, right_vec) = (
            Self::beautiful_array(odd_num),
            Self::beautiful_array(even_num),
        );
        // 将左侧数组映射为奇数
        for v in left_vec.iter() {
            result.push(v * 2 - 1);
        }
        // 将右侧数组映射为偶数
        for v in right_vec.iter() {
            result.push(v * 2);
        }
        result
    }
}
```

