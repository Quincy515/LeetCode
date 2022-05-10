# 198. 打家劫舍
你是一个专业的小偷，计划偷窃沿街的房屋。每间房内都藏有一定的现金，影响你偷窃的唯一制约因素就是相邻的房屋装有相互连通的防盗系统，**如果两间相邻的房屋在同一晚上被小偷闯入，系统会自动报警**。

给定一个代表每个房屋存放金额的非负整数数组，计算你**在不触动警报装置的情况下**，能够偷窃到的最高金额。

#### 示例 1:
<pre>
<strong>输入:</strong> [1,2,3,1]
<strong>输出:</strong> 4
<strong>解释:</strong> 偷窃 1 号房屋 (金额 = 1) ，然后偷窃 3 号房屋 (金额 = 3)。
     偷窃到的最高金额 = 1 + 3 = 4 。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> [2,7,9,3,1]
<strong>输出:</strong> 12
<strong>解释:</strong> 偷窃 1 号房屋 (金额 = 2), 偷窃 3 号房屋 (金额 = 9)，接着偷窃 5 号房屋 (金额 = 1)。
     偷窃到的最高金额 = 2 + 9 + 1 = 12 。
</pre>

## 题解
### Rust
```rust
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let n = nums.len();
        // dp[i][0] 表示第 i 个物品没有选时的最大金额
        // dp[i][1] 表示第 i 个物品选择时的最大金额
        let mut dp = vec![vec![0; 2]; n];
        dp[0][0] = 0;
        dp[0][1] = nums[0];
        for i in 1..n {
            dp[i][0] = i32::max(dp[i - 1][0], dp[i - 1][1]);
            dp[i][1] = dp[i - 1][0] + nums[i];
        }
        i32::max(dp[n - 1][0], dp[n - 1][1])
    }
}
```

### Go
```go
package main

import "math"

func rob(nums []int) int {
	if len(nums) == 0 {
		return 0
	}
	n := len(nums)
	dp := make([][]int, n)
	for i := 0; i < len(dp); i++ {
		dp[i] = make([]int, 2)
	}
	dp[0][0] = 0
	dp[0][1] = nums[0]
	for i := 1; i < n; i++ {
		dp[i][0] = int(math.Max(float64(dp[i-1][0]), float64(dp[i-1][1])))
		dp[i][1] = dp[i-1][0] + nums[i]
	}
	return int(math.Max(float64(dp[n-1][0]), float64(dp[n-1][1])))
}

```

### JavaScript
```javascript
/**
 * @param {number[]} nums
 * @return {number}
 */
var rob = function(nums) {
    let n = nums.length
    if (n == 0) {
        return 0
    }
    let dp = Array(n).fill(0).map(() => Array(2).fill(0))
    dp[0][0] = 0
    dp[0][1] = nums[0]
    for (let i = 1; i < n; ++i) {
        dp[i][0] = Math.max(dp[i-1][0], dp[i-1][1])
        dp[i][1] = dp[i-1][0] + nums[i]
    }
    return Math.max(dp[n-1][0], dp[n-1][1])
};
```
### Python
```python
class Solution:
    def rob(self, nums: List[int]) -> int:
        if len(nums) == 0:
            return 0
        n = len(nums)
        dp = [[0,0] for _ in range(n)]
        dp[0][0] = 0
        dp[0][1] = nums[0]
        for i in range(1,n):
            dp[i][0] = max(dp[i-1][0], dp[i-1][1])
            dp[i][1] = dp[i-1][0] + nums[i]
        return max(dp[n-1][0], dp[n-1][1])
    
```
### C++
```c++
class Solution {
public:
    int rob(vector<int> &nums) {
        if (nums.size() == 0) return 0;
        int n = nums.size();
        vector<vector<int>> dp(n, vector<int>(2));
        dp[0][0] = 0;
        dp[0][1] = nums[0];
        for (int i = 1; i < n; ++i) {
            dp[i][0] = std::max(dp[i - 1][0], dp[i - 1][1]);
            dp[i][1] = dp[i - 1][0] + nums[i];
        }
        return std::max(dp[n - 1][0], dp[n - 1][1]);
    }
};
```