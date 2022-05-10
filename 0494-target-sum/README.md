# 494. 目标和
给你一个整数数组 `nums` 和一个整数 `target` 。

向数组中的每个整数前添加 `'+'` 或 `'-'` ，然后串联起所有整数，可以构造一个 **表达式** ：
* 例如，`nums = [2, 1]` ，可以在 `2` 之前添加 `'+'` ，在 `1` 之前添加 `'-'` ，然后串联起来得到表达式 `"+2-1"` 。

返回可以通过上述方法构造的、运算结果等于 `target` 的不同 **表达式** 的数目。

#### 示例 1:
<pre>
<strong>输入:</strong> nums = [1,1,1,1,1], target = 3
<strong>输出:</strong> 5
<strong>解释:</strong> 一共有 5 种方法让最终目标和为 3 。
-1 + 1 + 1 + 1 + 1 = 3
+1 - 1 + 1 + 1 + 1 = 3
+1 + 1 - 1 + 1 + 1 = 3
+1 + 1 + 1 - 1 + 1 = 3
+1 + 1 + 1 + 1 - 1 = 3
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> nums = [1], target = 1
<strong>输出:</strong> 1
</pre>

#### 提示:
* `1 <= nums.length <= 20`
* `0 <= nums[i] <= 1000`
* `0 <= sum(nums[i]) <= 1000`
* `-1000 <= target <= 1000`

## 题解
### Rust
```rust
impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        if target > 1000 || target < -1000 {
            return 0;
        }
        let n = nums.len();
        let offset = 1000;
        let w = 2000;
        let mut dp = vec![vec![0; w + 1]; n];
        dp[0][(offset - nums[0]) as usize] += 1; // 因为 nums[0] 有可能为 0
        dp[0][(offset + nums[0]) as usize] += 1;
        for i in 1..n {
            for j in 0..=w {
                if j as i32 - nums[i] >= 0 && j as i32 - nums[i] <= w as i32 {
                    dp[i][j] = dp[i - 1][j - nums[i] as usize];
                }
                if j as i32 + nums[i] >= 0 && j as i32 + nums[i] <= w as i32 {
                    dp[i][j] += dp[i - 1][j + nums[i] as usize];
                }
            }
        }

        dp[n - 1][target as usize + 1000]
    }
}
```

### Go
```go
package main

func findTargetSumWays(nums []int, target int) int {
	if target > 1000 || target < -1000 {
		return 0
	}
	n := len(nums)
	offset := 1000
	w := 2000
	dp := make([][]int, n)
	for i := 0; i < len(dp); i++ {
		dp[i] = make([]int, w+1)
	}
	dp[0][offset-nums[0]] += 1
	dp[0][offset+nums[0]] += 1
	for i := 1; i < n; i++ {
		for j := 0; j <= w; j++ {
			if j-nums[i] >= 0 && j-nums[i] <= w {
				dp[i][j] = dp[i-1][j-nums[i]]
			}
			if j+nums[i] >= 0 && j+nums[i] <= w {
				dp[i][j] += dp[i-1][j+nums[i]]
			}
		}
	}
	return dp[n-1][target+1000]
}

```

### JavaScript
```javascript
/**
 * @param {number[]} nums
 * @param {number} target
 * @return {number}
 */
var findTargetSumWays = function(nums, target) {
    if (target > 1000 || target < -1000) {
        return 0
    }
    let n = nums.length
    let offset = 1000
    let w = 2000
    let dp = Array(n).fill().map(() => Array(w+1).fill(0))
    dp[0][offset-nums[0]] += 1
    dp[0][offset+nums[0]] += 1
    for (let i = 1; i < n; ++i) {
        for (let j = 0; j <= w; ++j) {
            if (j-nums[i] >= 0 && j-nums[i] <= w) {
                dp[i][j] = dp[i-1][j-nums[i]]
            }
            if (j+nums[i] >= 0 && j+nums[i] <= w) {
                dp[i][j] += dp[i-1][j+nums[i]]
            }
        }
    }
    return dp[n-1][target+1000]
};
```

### Python
```python
class Solution:
    def findTargetSumWays(self, nums: List[int], target: int) -> int:
        if target > 1000 or target < -1000:
            return 0
        n = len(nums)
        offset = 1000
        w = 2000
        dp = [[0] * (w + 1) for i in range(n)]
        dp[0][offset - nums[0]] += 1
        dp[0][offset + nums[0]] += 1
        for i in range(1, n):
            for j in range(w+1):
                if 0 <= j - nums[i] <= w:
                    dp[i][j] = dp[i-1][j-nums[i]]
                if 0 <= j + nums[i] <= w:
                    dp[i][j] += dp[i-1][j+nums[i]]
        return dp[n-1][target+offset]

```

### C++
```c++
class Solution {
public:
    int findTargetSumWays(vector<int> &nums, int target) {
        if (target > 1000 || target < -1000) return 0;
        int n = nums.size();
        int offset = 1000;
        int w = 2000;
        vector<vector<int>> dp(n, vector<int>(w + 1));
        dp[0][offset - nums[0]] += 1;
        dp[0][offset + nums[0]] += 1;
        for (int i = 1; i < n; ++i) {
            for (int j = 0; j <= w; ++j) {
                if (j - nums[i] >= 0 && j - nums[i] <= w) {
                    dp[i][j] = dp[i - 1][j - nums[i]];
                }
                if (j + nums[i] >= 0 && j + nums[i] <= w) {
                    dp[i][j] += dp[i - 1][j + nums[i]];
                }
            }
        }
        return dp[n - 1][target + 1000];
    }
};
```