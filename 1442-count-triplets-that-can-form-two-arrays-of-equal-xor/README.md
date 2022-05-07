# [1442.形成两个异或相等数组的三元组数目](https://leetcode-cn.com/problems/count-triplets-that-can-form-two-arrays-of-equal-xor/description/)

给你一个整数数组 `arr` 。

现需要从数组中取三个下标 `i`、`j` 和 `k` ，其中 `(0 <= i < j <= k < arr.length)` 。

`a` 和 `b` 定义如下：

- `a = arr[i] ^ arr[i + 1] ^ ... ^ arr[j - 1]`
- `b = arr[j] ^ arr[j + 1] ^ ... ^ arr[k]`

注意：**^** 表示 **按位异或** 操作。

请返回能够令 `a == b` 成立的三元组 (`i`, `j` , `k`) 的数目。

 

**示例 1：**

```
输入：arr = [2,3,1,6,7]
输出：4
解释：满足题意的三元组分别是 (0,1,2), (0,2,2), (2,3,4) 以及 (2,4,4)
```

**示例 2：**

```
输入：arr = [1,1,1,1,1]
输出：10
```

**示例 3：**

```
输入：arr = [2,3]
输出：0
```

**示例 4：**

```
输入：arr = [1,3,5,7,9]
输出：3
```

**示例 5：**

```
输入：arr = [7,11,12,9,5,2,7,17,22]
输出：8
```

 

**提示：**

- `1 <= arr.length <= 300`
- `1 <= arr[i] <= 10^8`

------

[Discussion](https://leetcode-cn.com/problems/count-triplets-that-can-form-two-arrays-of-equal-xor/comments/) | [Solution](https://leetcode-cn.com/problems/count-triplets-that-can-form-two-arrays-of-equal-xor/solution/)

**思路**

1、计算前缀和

2、枚举一个 **j**，再枚举 **i** 将异或和 **[i, j-1]** 放入哈希表

3、再枚举 **k**，去哈希表中查找 **[j, k]** 的异或和并且累加

4、时间复杂度可以做到 **O(n*n)**

**题解**

```rust
impl Solution {
    pub fn count_triplets(mut arr: Vec<i32>) -> i32 {
        let mut hash = std::collections::HashMap::new();
        let (mut pre, mut result) = (0, 0);
        for i in 1..arr.len() {
            arr[i] = arr[i - 1] ^ arr[i];
        }
        for j in 1..arr.len() {
            hash.clear();
            for i in 0..j {
                if i == 0 {
                    pre = 0;
                } else {
                    pre = arr[i - 1];
                }
                *hash.entry(arr[j - 1] ^ pre).or_insert(0) += 1;
            }
            for k in j..arr.len() {
                result += *hash.entry(arr[k] ^ arr[j - 1]).or_insert(0);
            }
        }
        result
    }
}
```

