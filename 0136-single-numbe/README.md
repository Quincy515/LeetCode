# [136.只出现一次的数字](https://leetcode-cn.com/problems/single-number/description/)

给定一个**非空**整数数组，除了某个元素只出现一次以外，其余每个元素均出现两次。找出那个只出现了一次的元素。

**说明：**

你的算法应该具有线性时间复杂度。 你可以不使用额外空间来实现吗？

**示例 1:**

```
输入: [2,2,1]
输出: 1
```

**示例 2:**

```
输入: [4,1,2,1,2]
输出: 4
```

------

[Discussion](https://leetcode-cn.com/problems/single-number/comments/) | [Solution](https://leetcode-cn.com/problems/single-number/solution/)

**思路**

1、任意两个相同数异或的结果为0，所以只要偶数个数异或后，结果都为0

2、异或满足交换律和结合律，所以把所有数异或以后，得到的数就是我们要求的那个只出现一次的数

**题解**

```rust
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        for num in nums {
            result ^= num
        }
        result
    }
}
```

