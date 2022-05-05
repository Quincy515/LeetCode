# [713.乘积小于 K 的子数组](https://leetcode-cn.com/problems/subarray-product-less-than-k/description/)


给你一个整数数组 `nums` 和一个整数 `k` ，请你返回子数组内所有元素的乘积严格小于 `k` 的连续子数组的数目。

 

**示例 1：**

```
输入：nums = [10,5,2,6], k = 100
输出：8
解释：8 个乘积小于 100 的子数组分别为：[10]、[5]、[2],、[6]、[10,5]、[5,2]、[2,6]、[5,2,6]。
需要注意的是 [10,5,2] 并不是乘积小于 100 的子数组。
```

**示例 2：**

```
输入：nums = [1,2,3], k = 0
输出：0
```

 

**提示:** 

- `1 <= nums.length <= 3 * 104`
- `1 <= nums[i] <= 1000`
- `0 <= k <= 106`

------

[Discussion](https://leetcode-cn.com/problems/subarray-product-less-than-k/comments/) | [Solution](https://leetcode-cn.com/problems/subarray-product-less-than-k/solution/)

**提示:** 

1、双指针，遍历数组，当左右指针指向同一个数字，判断当前数字和k的大小

- 小于 k，右指针向右移动一位，val 乘以当前右指针数字
- 大于k，左指针向右移动一位，val 变为当前左指针数字

2、当左右指针不想等，判断当前 val 和 k 的大小

- 小于 k，右指针向右移动一位，val 乘以当前右指针数字
- 大于 k，左指针向右移动一位，右指针移动到左指针，val 变为当前左指针数字

**题解** 

```rust
impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        let (mut left, mut right) = (0, 0);
        let mut result = 0;
        let mut val = i32::MAX;
        while left < nums.len() {
            if left == right && nums[left] < k {
                result += 1;
                if right < nums.len() - 1 {
                    right += 1;
                    val = nums[left] * nums[right];
                } else {
                    left += 1;
                }
            } else if  val < k {
                result += 1;
                if right < nums.len() - 1 {
                    right += 1;
                    val *= nums[right];
                } else {
                    left += 1;
                    right = left;
                }
            } else {
                left += 1;
                right = left;
            }
        }
        result
    }
}
```

