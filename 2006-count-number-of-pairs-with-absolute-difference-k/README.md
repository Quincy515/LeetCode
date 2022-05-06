# [2006.差的绝对值为 K 的数对数目](https://leetcode-cn.com/problems/count-number-of-pairs-with-absolute-difference-k/description/)


给你一个整数数组 `nums` 和一个整数 `k` ，请你返回数对 `(i, j)` 的数目，满足 `i < j` 且 `|nums[i] - nums[j]| == k` 。

`|x|` 的值定义为：

- 如果 `x >= 0` ，那么值为 `x` 。
- 如果 `x < 0` ，那么值为 `-x` 。

 

**示例 1：**

```
输入：nums = [1,2,2,1], k = 1
输出：4
解释：差的绝对值为 1 的数对为：
- [_1_,_2_,2,1]
- [_1_,2,_2_,1]
- [1,_2_,2,_1_]
- [1,2,_2_,_1_]
```

**示例 2：**

```
输入：nums = [1,3], k = 3
输出：0
解释：没有任何数对差的绝对值为 3 。
```

**示例 3：**

```
输入：nums = [3,2,1,5,4], k = 2
输出：3
解释：差的绝对值为 2 的数对为：
- [_3_,2,_1_,5,4]
- [_3_,2,1,_5_,4]
- [3,_2_,1,5,_4_]
```

 

**提示：**

- `1 <= nums.length <= 200`
- `1 <= nums[i] <= 100`
- `1 <= k <= 99`

------

[Discussion](https://leetcode-cn.com/problems/count-number-of-pairs-with-absolute-difference-k/comments/) | [Solution](https://leetcode-cn.com/problems/count-number-of-pairs-with-absolute-difference-k/solution/)

**思路** 

1、**|nums[i] - nums[j]| == k** 转换成两个式子

- **nums[i] == nums[j] - k**
- **nums[i] == nums[j] + k**

2、定义一个哈希表，枚举 **j**，分别查找 **nums[j] +k** 和 **nums[j] - k** 是否在哈希表中，并且累加计数

3、然后将 **nums[j]** 插入哈希表

**题解**

```rust
impl Solution {
    pub fn count_k_difference(nums: Vec<i32>, k: i32) -> i32 {
        let mut hash = std::collections::HashMap::new();
        let mut result = 0;

        for num in nums {
            let mut x = num + k;
            result += *hash.entry(x).or_insert(0);            
            x = num - k;
            result += *hash.entry(x).or_insert(0);
            *hash.entry(num).or_insert(0) += 1;
        }

        result
    }
}
```

