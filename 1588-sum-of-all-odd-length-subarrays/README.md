# [1588.所有奇数长度子数组的和](https://leetcode-cn.com/problems/sum-of-all-odd-length-subarrays/description/)

给你一个正整数数组 `arr` ，请你计算所有可能的奇数长度子数组的和。

**子数组** 定义为原数组中的一个连续子序列。

请你返回 `arr` 中 **所有奇数长度子数组的和** 。

 

**示例 1：**

```
输入：arr = [1,4,2,5,3]
输出：58
解释：所有奇数长度子数组和它们的和为：
[1] = 1
[4] = 4
[2] = 2
[5] = 5
[3] = 3
[1,4,2] = 7
[4,2,5] = 11
[2,5,3] = 10
[1,4,2,5,3] = 15
我们将所有值求和得到 1 + 4 + 2 + 5 + 3 + 7 + 11 + 10 + 15 = 58
```

**示例 2：**

```
输入：arr = [1,2]
输出：3
解释：总共只有 2 个长度为奇数的子数组，[1] 和 [2]。它们的和为 3 。
```

**示例 3：**

```
输入：arr = [10,11,12]
输出：66
```

 

**提示：**

- `1 <= arr.length <= 100`
- `1 <= arr[i] <= 1000`

------

[Discussion](https://leetcode-cn.com/problems/sum-of-all-odd-length-subarrays/comments/) | [Solution](https://leetcode-cn.com/problems/sum-of-all-odd-length-subarrays/solution/)

**思路**

1、首先定义前缀和 **sun**

2、枚举一个长度 **i**，枚举一个结束为止 **j**，利用差分就能得到 **sum[j-i+1, j]** 的元素和

3、统计加和所有 **i** 为奇数的元素和即可

**题解**

```rust
impl Solution {
    pub fn sum_odd_length_subarrays(mut arr: Vec<i32>) -> i32 {
        let mut result = 0;
        for i in 1..arr.len() {
            arr[i] += arr[i - 1];
        }
        for i in (1..=arr.len()).step_by(2) {
            for j in i - 1..arr.len() {
                // 以第 j 个数结尾并且长度为 i 的子数组
                let pre = if j as i32 - i as i32 == -1 {
                    0
                } else {
                    arr[j - i]
                };
                result += arr[j] - pre;
            }
        }

        result
    }
}
```

