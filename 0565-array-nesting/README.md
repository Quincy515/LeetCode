# [565.数组嵌套](https://leetcode.cn/problems/array-nesting/description/)

索引从`0`开始长度为`N`的数组`A`，包含`0`到`N - 1`的所有整数。找到最大的集合`S`并返回其大小，其中 `S[i] = {A[i], A[A[i]], A[A[A[i]]], ... }`且遵守以下的规则。

假设选择索引为`i`的元素`A[i]`为`S`的第一个元素，`S`的下一个元素应该是`A[A[i]]`，之后是`A[A[A[i]]]...` 以此类推，不断添加直到`S`出现重复的元素。

 

**示例 1:**

```
输入: A = [5,4,0,3,1,6,2]
输出: 4
解释: 
A[0] = 5, A[1] = 4, A[2] = 0, A[3] = 3, A[4] = 1, A[5] = 6, A[6] = 2.

其中一种最长的 S[K]:
S[0] = {A[0], A[5], A[6], A[2]} = {5, 6, 2, 0}
```

 

**提示：**

1. `N`是`[1, 20,000]`之间的整数。
2. `A`中不含有重复的元素。
3. `A`中的元素大小在`[0, N-1]`之间。

------

[Discussion](https://leetcode.cn/problems/array-nesting/comments/) | [Solution](https://leetcode.cn/problems/array-nesting/solution/)

**思路**

1、采用染色法，遍历所有节点，对没有被染色的节点进行访问，并且执行计数

2、取计数的最大值就是需要返回的答案

**题解**

```rust
impl Solution {
    pub fn array_nesting(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut result = 0;
        let mut cnt = 0;
        let mut hash = [false;200001];
        for i in 0..n {
            if !hash[i] {
                cnt = 0;
                Self::dfs(&nums, i, &mut hash, &mut cnt);
            }
            result = result.max(cnt);
        }
        result
    }

    fn dfs(nums: &[i32], u: usize, hash: &mut [bool; 200001], cnt: &mut i32) {
        if hash[u] {
            return;
        }
        *cnt += 1;
        hash[u] = true;
        Self::dfs(nums, nums[u] as usize, hash, cnt);
    }
}
```

