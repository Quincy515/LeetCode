# [1984.学生分数的最小差值](https://leetcode-cn.com/problems/minimum-difference-between-highest-and-lowest-of-k-scores/description/)


给你一个 **下标从 0 开始** 的整数数组 `nums` ，其中 `nums[i]` 表示第 `i` 名学生的分数。另给你一个整数 `k` 。

从数组中选出任意 `k` 名学生的分数，使这 `k` 个分数间 **最高分** 和 **最低分** 的 **差值** 达到 **最小化** 。

返回可能的 **最小差值** 。

 

**示例 1：**

```
输入：nums = [90], k = 1
输出：0
解释：选出 1 名学生的分数，仅有 1 种方法：
- [90] 最高分和最低分之间的差值是 90 - 90 = 0
可能的最小差值是 0
```

**示例 2：**

```
输入：nums = [9,4,1,7], k = 2
输出：2
解释：选出 2 名学生的分数，有 6 种方法：
- [9,4,1,7] 最高分和最低分之间的差值是 9 - 4 = 5
- [9,4,1,7] 最高分和最低分之间的差值是 9 - 1 = 8
- [9,4,1,7] 最高分和最低分之间的差值是 9 - 7 = 2
- [9,4,1,7] 最高分和最低分之间的差值是 4 - 1 = 3
- [9,4,1,7] 最高分和最低分之间的差值是 7 - 4 = 3
- [9,4,1,7] 最高分和最低分之间的差值是 7 - 1 = 6
可能的最小差值是 2
```

 

**提示：**

- `1 <= k <= nums.length <= 1000`
- `0 <= nums[i] <= 105`

------

[Discussion](https://leetcode-cn.com/problems/minimum-difference-between-highest-and-lowest-of-k-scores/comments/) | [Solution](https://leetcode-cn.com/problems/minimum-difference-between-highest-and-lowest-of-k-scores/solution/)

**思路**

1、排序

2、定义两个指针 **i** 和 **j**，**j** 不断往右移动的过程中，如果窗口长度超过了 **k**，则 **i** 向右移动一位

3、当窗口长度等于 **k** 的时候，记录 **nums[j] - nums[i]** 的最小值

**题解**

```rust
impl Solution {
    pub fn minimum_difference(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort();
        let (mut i, mut j) = (0i32, -1i32);
        let mut result = i32::MAX;
        while j < nums.len() as i32 - 1 {
            j += 1;
            if j - i + 1 > k {
                i += 1;
            }
            if j - i + 1 == k {
                result = result.min(nums[j as usize] - nums[i as usize]);
            }
        }
        result
    }
}
```

