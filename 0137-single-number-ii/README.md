# [137.只出现一次的数字 II](https://leetcode-cn.com/problems/single-number-ii/description/)

给你一个整数数组 `nums` ，除某个元素仅出现 **一次** 外，其余每个元素都恰出现 **三次 。**请你找出并返回那个只出现了一次的元素。

 

**示例 1：**

```
输入：nums = [2,2,3,2]
输出：3
```

**示例 2：**

```
输入：nums = [0,1,0,1,0,1,99]
输出：99
```

 

**提示：**

- `1 <= nums.length <= 3 * 104`
- `-231 <= nums[i] <= 231 - 1`
- `nums` 中，除某个元素仅出现 **一次** 外，其余每个元素都恰出现 **三次**

 

**进阶：**你的算法应该具有线性时间复杂度。 你可以不使用额外空间来实现吗？

------

[Discussion](https://leetcode-cn.com/problems/single-number-ii/comments/) | [Solution](https://leetcode-cn.com/problems/single-number-ii/solution/)

**思路**

1、这题将原本的两个数变成了三个数，于是用异或就行不通了

2、将每个数转换成二进制以后排列起来

3、对于每个位进行分别计算，如果加起来是3的倍数，说明只出现一次的数的对应位是0；否则是1

**题解**

```rust
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        for i in 0..32 {
            let mut sum = 0;
            for num in &nums {
                sum += (num >> i) & 1;
            }
            sum %= 3;
            if sum != 0 {
                result += 1 << i;
            }
        }
        result
    }
}
```

