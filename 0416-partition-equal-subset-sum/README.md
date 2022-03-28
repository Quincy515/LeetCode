# 416. 分割等和子集
给你一个 **只包含正整数** 的 **非空** 数组 `nums` 。请你判断是否可以将这个数组分割成两个子集，使得两个子集的元素和相等。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,5,11,5]
<strong>输出:</strong> true
<strong>解释:</strong> 数组可以分割成 [1, 5, 5] 和 [11] 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,2,3,5]
<strong>输出:</strong> false
<strong>解释:</strong> 数组不能分割成两个元素和相等的子集。
</pre>

#### 提示:
* `1 <= nums.length <= 200`
* `1 <= nums[i] <= 100`

## 题解 

### Rust
```rust
impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mut sum = 0;
        for i in 0..n {
            sum += nums[i];
        }
        if sum % 2 == 1 {
            return false;
        }
        sum /= 2;
        let mut dp = vec![vec![false; sum as usize + 1]; n];
        dp[0][0] = true;
        if nums[0] <= sum {
            dp[0][nums[0] as usize] = true;
        }
        for i in 1..n {
            for j in 0..=sum {
                if j - nums[i] >= 0 {
                    dp[i as usize][j as usize] = dp[i as usize - 1][j as usize]
                        || dp[i as usize - 1][(j - nums[i]) as usize];
                } else {
                    dp[i as usize][j as usize] = dp[i as usize - 1][j as usize];
                }
            }
        }
        dp[n as usize - 1][sum as usize]
    }
}
```

### Go
```go
package main

func canPartition(nums []int) bool {
	n := len(nums)
	sum := 0
	for i := 0; i < n; i++ {
		sum += nums[i]
	}
	if sum%2 == 1 {
		return false
	}
	sum /= 2
	dp := make([][]bool, n)
	for i := 0; i < len(dp); i++ {
		dp[i] = make([]bool, sum+1)
	}
	dp[0][0] = true
	if nums[0] <= sum {
		dp[0][nums[0]] = true
	}
	for i := 1; i < n; i++ {
		for j := 0; j <= sum; j++ {
			if j-nums[i] >= 0 {
				dp[i][j] = dp[i-1][j] || dp[i-1][j-nums[i]]
			} else {
				dp[i][j] = dp[i-1][j]
			}
		}
	}
	return dp[n-1][sum]
}

```

### JavaScript
```javascript
/**
 * @param {number[]} nums
 * @return {boolean}
 */
var canPartition = function(nums) {
    let n = nums.length
    let sum = 0
    sum = nums.reduce((sum, currentValue) => sum += currentValue, 0)
    if (sum % 2 == 1) {
        return false
    }
    sum = Math.floor(sum / 2)
    let dp = Array(n).fill().map(() => Array(sum+1).fill(false))
    dp[0][0] = true
    if (nums[0] <= sum) {
        dp[0][nums[0]] = true
    }
    for (let i = 1; i < n; ++i) {
        for (let j = 0; j <= sum; ++j) {
            if (j-nums[i] >= 0) {
                dp[i][j] = dp[i-1][j] || dp[i-1][j-nums[i]]
            } else {
                dp[i][j] = dp[i-1][j]
            }
        }
    }
    return dp[n-1][sum]
};
```

### Python
```python
class Solution:
    def canPartition(self, nums: List[int]) -> bool:
        n = len(nums)
        target = sum(nums)
        if target % 2:
            return False
        target = target // 2
        dp = [[False] * (target + 1) for i in range(n)]
        dp[0][0] = True
        if nums[0] <= target:
            dp[0][nums[0]] = True
        for i in range(1, n):
            for j in range(target+1):
                if j - nums[i] >= 0:
                    dp[i][j] = dp[i-1][j] or dp[i-1][j-nums[i]]
                else:
                    dp[i][j] = dp[i-1][j]
        return dp[n-1][target]
```

### C++
```c++
class Solution {
public:
    bool canPartition(vector<int> &nums) {
        int n = nums.size();
        int sum = 0;
        for (int i = 0; i < n; ++i) {
            sum += nums[i];
        }
        if (sum % 2 == 1) return false;
        sum /= 2;
        vector <vector<bool>> dp(n, vector<bool>(sum + 1));
        dp[0][0] = true;
        if (nums[0] <= sum) {
            dp[0][nums[0]] = true;
        }
        for (int i = 1; i < n; ++i) {
            for (int j = 0; j <= sum; ++j) {
                if (j - nums[i] >= 0) {
                    dp[i][j] = dp[i - 1][j] || dp[i - 1][j - nums[i]];
                } else {
                    dp[i][j] = dp[i - 1][j];
                }
            }
        }
        return dp[n - 1][sum];
    }
};
```