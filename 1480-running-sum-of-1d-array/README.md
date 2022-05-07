# [1480.一维数组的动态和](https://leetcode-cn.com/problems/running-sum-of-1d-array/description/)

给你一个数组 `nums` 。数组「动态和」的计算公式为：`runningSum[i] = sum(nums[0]…nums[i])` 。

请返回 `nums` 的动态和。

 

**示例 1：**

```
输入：nums = [1,2,3,4]
输出：[1,3,6,10]
解释：动态和计算过程为 [1, 1+2, 1+2+3, 1+2+3+4] 。
```

**示例 2：**

```
输入：nums = [1,1,1,1,1]
输出：[1,2,3,4,5]
解释：动态和计算过程为 [1, 1+1, 1+1+1, 1+1+1+1, 1+1+1+1+1] 。
```

**示例 3：**

```
输入：nums = [3,1,2,10,1]
输出：[3,4,6,16,17]
```

 

**提示：**

- `1 <= nums.length <= 1000`
- `-10^6 <= nums[i] <= 10^6`

------

[Discussion](https://leetcode-cn.com/problems/running-sum-of-1d-array/comments/) | [Solution](https://leetcode-cn.com/problems/running-sum-of-1d-array/solution/)

**思路**

1、定义一个用于返回的数组 **result**

2、遍历原数组，如果第 0 个元素，则直接将它插入待返回的数组

3、否则，将待返回的数组的最后一个元素和当前第 **i** 个元素加和后插入

4、当然，也可以直接在原数组上操作

**题解**

```rust
impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();
        for i in 0..nums.len() {
            if i == 0 {
                result.push(nums[i]);
            } else {
                result.push(result[i - 1] + nums[i]);
            }
        }
        result
    }
}
```



```rust
impl Solution {
    pub fn running_sum(mut nums: Vec<i32>) -> Vec<i32> {
        for i in 1..nums.len() {
            nums[i] += nums[i - 1];
        }
        nums
    }
}
```

