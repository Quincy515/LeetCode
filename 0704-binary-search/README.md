# [704.二分查找](https://leetcode-cn.com/problems/binary-search/description/)

给定一个 `n` 个元素有序的（升序）整型数组 `nums` 和一个目标值 `target` ，写一个函数搜索 `nums` 中的 `target`，如果目标值存在返回下标，否则返回 `-1`。


**示例 1:**

```
输入: nums = [-1,0,3,5,9,12], target = 9
输出: 4
解释: 9 出现在 nums 中并且下标为 4
```

**示例 2:**

```
输入: nums = [-1,0,3,5,9,12], target = 2
输出: -1
解释: 2 不存在 nums 中因此返回 -1
```

 

**提示：**

1. 你可以假设 `nums` 中的所有元素是不重复的。
2. `n` 将在 `[1, 10000]`之间。
3. `nums` 的每个元素都将在 `[-9999, 9999]`之间。

------

[Discussion](https://leetcode-cn.com/problems/binary-search/comments/) | [Solution](https://leetcode-cn.com/problems/binary-search/solution/)

**思路**

同 [35. 搜索插入位置](https://leetcode.cn/problems/search-insert-position/) 思路 [链接](../../0035-search-insert-position)

**题解**

```rust
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let index = Self::bsearch(nums.clone(), target);
        if index == nums.len() as i32 || nums[index as usize] != target {
            return -1
        }
        index
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

