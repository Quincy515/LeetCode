# [剑指 Offer 53 - I. 在排序数组中查找数字 I](https://leetcode.cn/problems/zai-pai-xu-shu-zu-zhong-cha-zhao-shu-zi-lcof/)

统计一个数字在排序数组中出现的次数。

 

**示例 1:**

```
输入: nums = [5,7,7,8,8,10], target = 8
输出: 2
```

**示例 2:**

```
输入: nums = [5,7,7,8,8,10], target = 6
输出: 0
```

**提示：**

- 0 <= nums.length <= 105
- -109 <= nums[i] <= 109
- nums 是一个非递减数组
- -109 <= target <= 109


注意：本题与主站 34 题相同（仅返回值不同）：https://leetcode-cn.com/problems/find-first-and-last-position-of-element-in-sorted-array/

**思路**

1、通过二分查找先找大于等于 target 的最小下标，找到就返回下标 index，未找到返回 -1

2、如果未找到大于等于 target 的最小下标，直接返回 0

2、否则，通过二分查找找到大于等于 target+1 的最小下标，相减得出结果

**题解** 

```rust
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let index = Self::bsearch(nums.clone(), target);
        if index == nums.len() as i32 || nums[index as usize] != target {
            return 0;
        }
        Self::bsearch(nums, target + 1) - index
    }
    
    // 大于等于 target 的最小下标
    fn bsearch(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return nums.len() as i32;
        }
        let (mut left, mut right) = (0, nums.len());
        let mut result = nums.len();
        while left < right {
            let mid = (left + right) >> 1;
            if target <= nums[mid] {
                result = mid;
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        result as i32
    }
}
```

