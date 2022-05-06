# [1512.好数对的数目](https://leetcode-cn.com/problems/number-of-good-pairs/description/)


给你一个整数数组 `nums` 。

如果一组数字 `(i,j)` 满足 `nums[i]` == `nums[j]` 且 `i` < `j` ，就可以认为这是一组 **好数对** 。

返回好数对的数目。

 

**示例 1：**

```
输入：nums = [1,2,3,1,1,3]
输出：4
解释：有 4 组好数对，分别是 (0,3), (0,4), (3,4), (2,5) ，下标从 0 开始
```

**示例 2：**

```
输入：nums = [1,1,1,1]
输出：6
解释：数组中的每组数字都是好数对
```

**示例 3：**

```
输入：nums = [1,2,3]
输出：0
```

 

**提示：**

- `1 <= nums.length <= 100`
- `1 <= nums[i] <= 100`

------

[Discussion](https://leetcode-cn.com/problems/number-of-good-pairs/comments/) | [Solution](https://leetcode-cn.com/problems/number-of-good-pairs/solution/)

**思路** 

1、假设有一种容器，我们可以枚举一个 **j**，容器里面放的是所有 **[0, j-1]** 的数字计数

2、当我们执行查找的时候，能够快速找到这个数，并且将它进行统计；而且插入也能达到理想时间复杂度

3、符合这两个操作的容器，就是哈希表 

**题解**

```rust
impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut hash = std::collections::HashMap::new();
        for i in 0..nums.len() {
            let count = hash.entry(nums[i as usize]).or_insert(0);
            result += *count; // 查找
            *count += 1; // 插入
        }

        result
    }
}
```

