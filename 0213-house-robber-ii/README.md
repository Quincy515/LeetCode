# 213. 打家劫舍 II
你是一个专业的小偷，计划偷窃沿街的房屋，每间房内都藏有一定的现金。这个地方所有的房屋都 **围成一圈** ，这意味着第一个房屋和最后一个房屋是紧挨着的。同时，相邻的房屋装有相互连通的防盗系统，**如果两间相邻的房屋在同一晚上被小偷闯入，系统会自动报警** 。

给定一个代表每个房屋存放金额的非负整数数组，计算你 **在不触动警报装置的情况下** ，能够偷窃到的最高金额。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [2,3,2]
<strong>输出:</strong> 3
<strong>解释:</strong> 你不能先偷窃 1 号房屋（金额 = 2），然后偷窃 3 号房屋（金额 = 2）, 因为他们是相邻的。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1,2,3,1]
<strong>输出:</strong> 4
<strong>解释:</strong> 你可以先偷窃 1 号房屋（金额 = 1），然后偷窃 3 号房屋（金额 = 3）。
     偷窃到的最高金额 = 1 + 3 = 4 。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> nums = [0]
<strong>输出:</strong> 0
</pre>

#### 提示:
* `1 <= nums.length <= 100`
* `0 <= nums[i] <= 1000`

## 题解
### Rust
```rust
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 1 {
            return nums[0];
        }
        if n == 2 {
            return i32::max(nums[0], nums[1]);
        }

        // 第 0 个不偷窃，偷窃 1~n-1 之间的房子
        let max1 = Self::rob_dp(&nums, 1, n - 1);
        // 第 0 个偷窃，偷窃 2~n-1 之间的房子
        let max2 = nums[0] + Self::rob_dp(&nums, 2, n - 2);
        i32::max(max1, max2)
    }

    fn rob_dp(nums: &[i32], p: usize, r: usize) -> i32 {
        let n = nums.len();
        // dp[i][0] 表示第 i 个物品美誉哦选时的最大金额
        // dp[i][1] 表示第 i 个物品选择时的最大金额
        let mut dp = vec![vec![0; 2]; n];
        dp[p][0] = 0;
        dp[p][1] = nums[p];
        for i in p + 1..=r {
            dp[i][0] = i32::max(dp[i - 1][0], dp[i - 1][1]);
            dp[i][1] = dp[i - 1][0] + nums[i];
        }
        i32::max(dp[r][0], dp[r][1])
    }
}
```

### Go
```go
package main

import "math"

func rob(nums []int) int {
	n := len(nums)
	if n == 1 {
		return nums[0]
	}
	if n == 2 {
		return int(math.Max(float64(nums[0]), float64(nums[1])))
	}
	max1 := rob_dp(nums, 1, n-1)
	max2 := nums[0] + rob_dp(nums, 2, n-2)
	return int(math.Max(float64(max1), float64(max2)))
}
func rob_dp(nums []int, p, r int) int {
	n := len(nums)
	dp := make([][]int, n)
	for i := 0; i < len(dp); i++ {
		dp[i] = make([]int, 2)
	}
	dp[p][0] = 0
	dp[p][1] = nums[p]
	for i := p + 1; i <= r; i++ {
		dp[i][0] = int(math.Max(float64(dp[i-1][0]), float64(dp[i-1][1])))
		dp[i][1] = dp[i-1][0] + nums[i]
	}
	return int(math.Max(float64(dp[r][0]), float64(dp[r][1])))
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
    if (n == 1) {
        return nums[0]
    }
    if (n == 2) {
        return Math.max(nums[0], nums[1])
    }
    let rob_dp = (p, r) => {
        let dp = Array(n).fill().map(() => Array(2).fill(0))
        dp[p][0] = 0
        dp[p][1] = nums[p]
        for (let i = p+1; i <= r; ++i) {
            dp[i][0] = Math.max(dp[i-1][0], dp[i-1][1])
            dp[i][1] = dp[i-1][0] + nums[i]
        }
        return Math.max(dp[r][0], dp[r][1])
    }
    let max1 = rob_dp(1, n-1)
    let max2 = nums[0] + rob_dp(2, n-2)
    return Math.max(max1, max2)
};
```

### Python
```python
class Solution:
    def rob(self, nums: List[int]) -> int:
        n = len(nums)
        if n == 1:
            return nums[0]
        if n == 2:
            return max(nums[0], nums[1])
        max1 = self.rob_dp(nums, 1,n-1)
        max2 = self.rob_dp(nums, 2,n-2) + nums[0]
        return max(max1, max2)

    def rob_dp(self,nums,p,r):
        n = len(nums)
        dp = [[0,0] for _ in range(n)]
        dp[p][0] = 0
        dp[p][1] = nums[p]
        for i in range(p+1, r+1):
            dp[i][0] = max(dp[i-1][0], dp[i-1][1])
            dp[i][1] = dp[i-1][0] + nums[i]
        return max(dp[r][0], dp[r][1])

```

### C++
```c++
class Solution {
public:
    int rob(vector<int> &nums) {
        int n = nums.size();
        if (n == 1) return nums[0];
        if (n == 2) return std::max(nums[0], nums[1]);
        int max1 = rob_dp(nums, 1, n - 1);
        int max2 = nums[0] + rob_dp(nums, 2, n - 2);
        return std::max(max1, max2);
    }
    int rob_dp(vector<int> nums, int p, int r) {
        int n = nums.size();
        vector<vector<int>> dp(n, vector<int>(2));
        dp[p][0] = 0;
        dp[p][1] = nums[p];
        for (int i = p + 1; i <= r; ++i) {
            dp[i][0] = std::max(dp[i - 1][0], dp[i - 1][1]);
            dp[i][1] = dp[i - 1][0] + nums[i];
        }
        return std::max(dp[r][0], dp[r][1]);
    }
};
```