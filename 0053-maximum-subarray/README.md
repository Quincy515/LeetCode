# 53. 最大子序和
给定一个整数数组 ```nums``` ，找到一个具有最大和的连续子数组（子数组最少包含一个元素），返回其最大和。

#### 示例:
<pre>
<strong>输入:</strong> [-2,1,-3,4,-1,2,1,-5,4],
<strong>输出:</strong> 6
<strong>解释:</strong> 连续子数组 [4,-1,2,1] 的和最大，为 6。
</pre>

#### 进阶:
如果你已经实现复杂度为 O(*n*) 的解法，尝试使用更为精妙的分治法求解。

## 题解：
### Rust
```rust


impl Solution {
    // 滑动窗口
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut max_sum = i32::MIN;
        let mut sum = 0;
        for i in 0..n {
            if sum < 0 {
                sum = 0;
            }
            sum += nums[i];
            if sum > max_sum {
                max_sum = sum;
            }
        }
        max_sum
    }
    // 前后缀
    pub fn max_sub_array2(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }
        let (mut sum, mut max) = (vec![0; nums.len()], vec![0; nums.len()]);

        let mut cursum = 0;
        for i in 0..nums.len() {
            cursum += nums[i];
            sum[i] = cursum;
        }
        let mut curmax = i32::MIN;
        for i in (0..=sum.len() - 1).rev() {
            if curmax < sum[i] {
                curmax = sum[i];
            }
            max[i] = curmax;
        }
        let mut result = i32::MIN;
        for i in 0..nums.len() {
            if i == 0 && result < max[0] {
                result = max[0];
            }
            if i != 0 && result < max[i] - sum[i - 1] {
                result = max[i] - sum[i - 1];
            }
        }
        result
    }
}
```

### Go
```go
package main

import "math"

//滑动窗口
func maxSubArray(nums []int) int {
	n := len(nums)
	maxSum := math.MinInt32
	sum := 0
	for i := 0; i < n; i++ {
		if sum < 0 {
			sum = 0
		}
		sum += nums[i]
		if sum > maxSum {
			maxSum = sum
		}
	}
	return maxSum
} 

//前后缀
func maxSubArray(nums []int) int {
	if len(nums) == 1 {
		return nums[0]
	}
	sum := make([]int, len(nums))
	max := make([]int, len(nums))
	cursum := 0
	for i := 0; i < len(nums); i++ {
		cursum += nums[i]
		sum[i] = cursum
	}
	curmax := math.MinInt32
	for i := len(sum) - 1; i >= 0; i-- {
		if curmax < sum[i] {
			curmax = sum[i]
		}
		max[i] = curmax
	}
	result := math.MinInt32
	for i := 0; i < len(nums); i++ {
		if i == 0 && result < max[0] {
			result = max[0]
		}
		if i != 0 && result < max[i]-sum[i-1] {
			result = max[i] - sum[i-1]
		}
	}
	return result
}

```

### JavaScript
```javascript
/**
 滑动窗⼝
 * @param {number[]} nums
 * @return {number}
 */
var maxSubArray = function(nums) {
    let n = nums.length
    let maxSum = Number.MIN_SAFE_INTEGER
    let sum = 0
    for (let i = 0; i < n; ++i) {
        if (sum < 0) {
            sum = 0
        }
        sum += nums[i]
        if (sum > maxSum) {
            maxSum = sum
        }
    }
    return maxSum
};

/**
 前缀后缀统计
 * @param {number[]} nums
 * @return {number}
 */
var maxSubArray = function(nums) {
    if (nums.length == 1) {
        return nums[0]
    }
    let sum = new Array(nums.length)
    let max = new Array(nums.length)
    let cursum = 0
    for (let i = 0; i < nums.length; ++i) {
        cursum += nums[i]
        sum[i] = cursum
    }
    let curmax = Number.MIN_SAFE_INTEGER
    for (let i = sum.length-1; i >= 0; --i) {
        if (curmax < sum[i]) {
            curmax = sum[i]
        }
        max[i] = curmax
    }
    let result = Number.MIN_SAFE_INTEGER
    for (let i = 0; i < nums.length; ++i) {
        if (i == 0 && result < max[0]) {
            result = max[0]
        }
        if (i != 0 && result < max[i]-sum[i-1]) {
            result = max[i] - sum[i-1]
        }
    }
    return result
};
```

### Python
```python
class Solution:
    def maxSubArray(self, nums: List[int]) -> int:
        n = len(nums)
        maxSum = float("-inf")
        total = 0
        for i in range(n):
            if total < 0:
                total = 0
            total += nums[i]
            if total > maxSum:
                maxSum = total
        return maxSum


class Solution:
    def maxSubArray(self, nums: List[int]) -> int:
        if len(nums) == 1:
            return nums[0]
        n = len(nums)
        totalSum = [0] * n
        maxCur = [0] * n
        cursum = 0
        for i in range(n):
            cursum += nums[i]
            totalSum[i]= cursum
        cursum = float("-inf")
        for i in range(n-1, -1,-1):
            if cursum < totalSum[i]:
                cursum = totalSum[i]
            maxCur[i] = cursum
        result = float("-inf")
        for i in range(n):
            if i == 0 and result < maxCur[0]:
                result = maxCur[0]
            if i != 0 and result < maxCur[i] - totalSum[i-1]:
                result = maxCur[i] - totalSum[i-1]
        return result

```

### C++
```c++
class Solution {
public:
    int maxSubArray(vector<int>& nums) {
        int n = nums.size();
        int maxSum = INT_MIN;
        int sum = 0;
        for (int i = 0; i < n; ++i) {
            if (sum < 0) {
                sum = 0;
            }
            sum += nums[i];
            if (sum > maxSum) {
                maxSum = sum;
            }
        }
        return maxSum;
    }
};

class Solution {
public:
    int maxSubArray(vector<int>& nums) {
        if (nums.size() == 1) return nums[0];
        vector<int> sum(nums.size());
        vector<int> max(nums.size());
        int cursum = 0;
        for (int i = 0; i < nums.size(); ++i) {
            cursum += nums[i];
            sum[i] = cursum;
        }
        int curmax = INT_MIN;
        for (int i = sum.size() - 1; i >= 0; --i) {
            if (curmax < sum[i]) curmax = sum[i];
            max[i] = curmax;
        }
        int result = INT_MIN;
        for (int i = 0; i < nums.size(); ++i) {
            if (i == 0 && result < max[0]) {
                result = max[0];
            }
            if (i != 0 && result < max[i] - sum[i - 1]) {
                result = max[i] - sum[i - 1];
            }
        }
        return result;
    }
}
```