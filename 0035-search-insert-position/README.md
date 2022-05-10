# [35.搜索插入位置](https://leetcode-cn.com/problems/search-insert-position/description/)

给定一个排序数组和一个目标值，在数组中找到目标值，并返回其索引。如果目标值不存在于数组中，返回它将会被按顺序插入的位置。

请必须使用时间复杂度为 `O(log n)` 的算法。

 

**示例 1:**

```
输入: nums = [1,3,5,6], target = 5
输出: 2
```

**示例 2:**

```
输入: nums = [1,3,5,6], target = 2
输出: 1
```

**示例 3:**

```
输入: nums = [1,3,5,6], target = 7
输出: 4
```

 

**提示:**

- `1 <= nums.length <= 104`
- `-104 <= nums[i] <= 104`
- `nums` 为 **无重复元素** 的 **升序** 排列数组
- `-104 <= target <= 104`

------

[Discussion](https://leetcode-cn.com/problems/search-insert-position/comments/) | [Solution](https://leetcode-cn.com/problems/search-insert-position/solution/)

**思路**

实现一个函数，代表从一个数组中找到大于等于 target 的最小下标

- 用一个 ans 来存储这个下标，一开始等于数组的右边界，代表如果不存在大于等于 target 函数，则下标就是这个右边界，即右边界是一个无限大的函数
- nums[mid]>=target 表示 mid 是一个满足条件的解，所以我们把 mid 存储下来，继续在 mid 左边尝试找更小的。

**题解**

```rust
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        Self::bsearch(nums, target)
    }

    // 找大于等于 target 的最小下标
    fn bsearch(nums: Vec<i32>, target: i32) -> i32 {
        let mut result = nums.len();
        if nums.is_empty() {
            return result as i32;
        }

        let (mut left, mut right) = (0, nums.len());
        while left < right {
            let mid = left + (right - left) / 2;
            if nums[mid] >= target {
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

使用库函数

```rust
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        match nums.binary_search(&target) {
            Ok(i) => i as i32,
            Err(i) => i as i32,
        }
    }
}
```

