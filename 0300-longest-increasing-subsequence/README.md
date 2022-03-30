# 300. 最长递增子序列

给你一个整数数组 nums ，找到其中最长严格递增子序列的长度。

子序列 是由数组派生而来的序列，删除（或不删除）数组中的元素而不改变其余元素的顺序。例如，[3,6,2,7] 是数组 [0,3,1,6,2,2,7] 的子序列。


## 示例 1：
```
输入：nums = [10,9,2,5,3,7,101,18]
输出：4
解释：最长递增子序列是 [2,3,7,101]，因此长度为 4 。
```

## 示例 2：
```
输入：nums = [0,1,0,3,2,3]
输出：4
```

## 示例 3：
```
输入：nums = [7,7,7,7,7,7,7]
输出：1
```

## 提示：

- 1 <= nums.length <= 2500
- -104 <= nums[i] <= 104


进阶：

你能将算法的时间复杂度降低到 O(n log(n)) 吗?

## 题解
### Rust
```rust
impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp = vec![0; n];
        dp[0] = 1;
        for i in 1..n {
            dp[i] = 1;
            for j in 0..i {
                if nums[i] > nums[j] {
                    dp[i] = i32::max(dp[i], dp[j] + 1);
                }
            }
        }

        let mut result = 0;
        for i in 0..n {
            if dp[i] > result {
                result = dp[i];
            }
        }
        result
    }

    pub fn length_of_lis2(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut list_to_min_v = vec![0; n + 1];
        let mut k = 0i32;

        let mut dp = vec![0; n];
        for i in 0..n {
            let len = Self::bsearch(&mut list_to_min_v, k, nums[i]);
            if len == -1 {
                dp[i] = 1;
            } else {
                dp[i] = len + 1;
            }
            if dp[i] > k {
                k = dp[i];
                list_to_min_v[dp[i] as usize] = nums[i];
            } else if list_to_min_v[dp[i] as usize] > nums[i] {
                list_to_min_v[dp[i] as usize] = nums[i];
            }
        }

        let mut result = 0;
        for i in 0..n {
            if dp[i] > result {
                result = dp[i];
            }
        }
        result
    }

    // 查找最后一个比 target 小的元素位置
    fn bsearch(a: &mut Vec<i32>, k: i32, target: i32) -> i32 {
        let (mut low, mut high) = (1, k);
        while low <= high {
            let mid = low + (high - low) / 2;
            if a[mid as usize] < target {
                if mid == k || a[mid as usize + 1] >= target {
                    return mid;
                } else {
                    low = mid + 1;
                }
            } else {
                high = mid - 1;
            }
        }
        -1
    }
}
```

### Go
```go
package main

import "math"

func lengthOfLIS(nums []int) int {
	n := len(nums)
	dp := make([]int, n)
	dp[0] = 1
	for i := 1; i < n; i++ {
		dp[i] = 1
		for j := 0; j < i; j++ {
			if nums[i] > nums[j] {
				dp[i] = int(math.Max(float64(dp[i]), float64(dp[j]+1)))
			}
		}
	}
	result := 0
	for i := 0; i < n; i++ {
		if dp[i] > result {
			result = dp[i]
		}
	}
	return result
}

//解法 2
func lengthOfLIS2(nums []int) int {
	n := len(nums)
	lisToMinV := make([]int, n+1)
	k := 0
	dp := make([]int, n)
	for i := 0; i < n; i++ {
		len := bsearch(lisToMinV, k, nums[i])
		if len == -1 {
			dp[i] = 1
		} else {
			dp[i] = len + 1
		}
		if dp[i] > k {
			k = dp[i]
			lisToMinV[dp[i]] = nums[i]
		} else if lisToMinV[dp[i]] > nums[i] {
			lisToMinV[dp[i]] = nums[i]
		}
	}
	result := 0
	for i := 0; i < n; i++ {
		if dp[i] > result {
			result = dp[i]
		}
	}
	return result
}

//查找最后一个比 target 小的元素位置
func bsearch(a []int, k, target int) int {
	low := 1
	high := k
	for low <= high {
		mid := (low + high) / 2
		if a[mid] < target {
			if mid == k || a[mid+1] >= target {
				return mid
			} else {
				low = mid + 1
			}
		} else {
			high = mid - 1
		}
	}
	return -1
}

```

### JavaScript
```javascript
/**
 * @param {number[]} nums
 * @return {number}
 */
var lengthOfLIS = function(nums) {
    let n = nums.length
    let dp = new Array(n).fill(0)
    dp[0] = 1
    for (let i = 1; i < n; ++i) {
        dp[i] = 1
        for (let j = 0; j < i; ++j) {
            if (nums[i] > nums[j]) {
                dp[i] = Math.max(dp[i], dp[j] + 1)
            }
        }
    }
    return Math.max(...dp)
};
/**
 * @param {number[]} nums
 * @return {number}
 */
var lengthOfLIS = function(nums) {
    let n = nums.length
    let lisToMinV = new Array(n+1).fill(0)
    let k = 0
    let dp = new Array(n).fill(0)
    for (let i = 0; i < n; ++i) {
        let len = bsearch(lisToMinV, k, nums[i])
        if (len == -1) {
            dp[i] = 1
        } else {
            dp[i] = len + 1
        }
        if (dp[i] > k) {
            k = dp[i]
            lisToMinV[dp[i]] = nums[i]
        } else if (lisToMinV[dp[i]] > nums[i]) {
            lisToMinV[dp[i]] = nums[i]
        }
    }
    return Math.max(...dp)
};
var bsearch = (a, k, target) => {
    let low = 1
    let high = k
    while (low <= high) {
        let mid = Math.floor((low+high)/2)
        if (a[mid] < target) {
            if (mid == k || a[mid+1] >= target) {
                return mid
            } else {
                low = mid + 1
            }
        } else {
            high = mid - 1
        }
    }
    return -1 }
```

### Python
```python
class Solution:
    def lengthOfLIS(self, nums: List[int]) -> int:
        n = len(nums)
        dp = [0] * n
        dp[0] = 1
        for i in range(1,n):
            dp[i] = 1
            for j in range(i):
                if nums[i] > nums[j]:
                    dp[i] = max(dp[i], dp[j] + 1)
        result = 0
        for i in range(n):
            if dp[i] > result:
                result = dp[i]
        return result

class Solution:
    def lengthOfLIS(self, nums: List[int]) -> int:
        n = len(nums)
        listToMinV = [float(inf)] * (n+1)
        k = 0
        dp = [0] * n
        for i in range(n):
            length = self.bsearch(listToMinV,k, nums[i])
            if length == -1:
                dp[i] = 1
            else:
                dp[i] = length+1
            if dp[i] > k:
                k = dp[i]
                listToMinV[dp[i]] = nums[i]
            elif listToMinV[dp[i]] > nums[i]:
                listToMinV[dp[i]] = nums[i]
        result = 0
        for i in range(n):
            if dp[i] > result:
                result = dp[i]
        return result

    def bsearch(self, a, k, target):
        low = 1
        high = k
        while low <= high:
            mid = (low + high) // 2
            if a[mid] < target:
                if mid == k or a[mid+1] >= target:
                    return mid
                else:
                    low = mid + 1
            else:
                high = mid - 1
        return -1
```

### C++
```c++
class Solution {
public:
    int lengthOfLIS(vector<int>& nums) {
        int n = nums.size();
        vector<int> dp(n);
        dp[0] = 1;
        for (int i = 1; i < n; ++i) {
            dp[i] = 1;
            for (int j = 0; j < i; ++j) {
                if (nums[i] > nums[j]) {
                    dp[i] = std::max(dp[i], dp[j] + 1);
                }
            }
        }
        int result = 0;
        for (int i = 0; i < n; ++i) {
            if (dp[i] > result) result = dp[i];
        }
        return result;
    }
};
```