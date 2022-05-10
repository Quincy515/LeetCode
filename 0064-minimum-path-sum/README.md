# 64. 最小路径和
给定一个包含非负整数的 *m* x *n* 网格，请找出一条从左上角到右下角的路径，使得路径上的数字总和为最小。

**说明:** 每次只能向下或者向右移动一步。

#### 示例:
<pre>
<strong>输入:</strong>
[
  [1,3,1],
  [1,5,1],
  [4,2,1]
]
<strong>输出:</strong> 7
<strong>解释:</strong> 因为路径 1→3→1→1→1 的总和最小。
</pre>
## 题解 

### Rust
```rust
impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid.first().map_or(0, Vec::len));
        let mut dp = vec![vec![0; n]; m];
        let mut len = 0;
        for i in 0..m {
            len += grid[i][0];
            dp[i][0] = len;
        }
        len = 0;
        for j in 0..n {
            len += grid[0][j];
            dp[0][j] = len;
        }

        for i in 1..m {
            for j in 1..n {
                dp[i][j] = i32::min(dp[i - 1][j], dp[i][j - 1]) + grid[i][j];
            }
        }

        dp[m - 1][n - 1]
    }
}
```

### Go
```go
package main

import "math"

func minPathSum(grid [][]int) int {
	m := len(grid)
	n := len(grid[0])
	dp := make([][]int, m)
	for i := 0; i < m; i++ {
		dp[i] = make([]int, n)
	}
	len := 0
	for i := 0; i < m; i++ {
		len += grid[i][0]
		dp[i][0] = len
	}
	len = 0
	for j := 0; j < n; j++ {
		len += grid[0][j]
		dp[0][j] = len
	}
	for i := 1; i < m; i++ {
		for j := 1; j < n; j++ {
			dp[i][j] = int(math.Min(float64(dp[i-1][j]), float64(dp[i][j-1]))) + grid[i][j]
		}
	}
	return dp[m-1][n-1]
}

```

### JavaScript
```javascript
/**
 * @param {number[][]} grid
 * @return {number}
 */
var minPathSum = function(grid) {
    let m = grid.length
    let n = grid[0].length
    let dp = Array(m).fill().map(() => Array(n).fill(0))
    let len = 0
    for (let i = 0; i < m; ++i) {
        len += grid[i][0]
        dp[i][0] = len
    }
    len = 0
    for (let j = 0; j < n; ++j) {
        len += grid[0][j]
        dp[0][j] = len
    }
    for (let i = 1; i < m; ++i) {
        for (let j = 1; j < n; ++j) {
            dp[i][j] = Math.min(dp[i-1][j], dp[i][j-1]) + grid[i][j]
        }
    }
    return dp[m-1][n-1]
};
```

### Python
```python
class Solution:
    def minPathSum(self, grid: List[List[int]]) -> int:
        m = len(grid)
        n = len(grid[0])
        dp = [[0] * n for i in range(m)]
        step = 0
        for i in range(m):
            step += grid[i][0]
            dp[i][0] = step
        step = 0
        for j in range(n):
            step += grid[0][j]
            dp[0][j] = step
        for i in range(1,m):
            for j in range(1, n):
                dp[i][j] = min(dp[i-1][j], dp[i][j-1]) + grid[i][j]
        return dp[m-1][n-1]

```

### C++
```c++
class Solution {
public:
    int minPathSum(vector <vector<int>> &grid) {
        int m = grid.size();
        int n = grid[0].size();
        vector<vector<int>> dp(m, vector<int>(n));
        int len = 0;
        for (int i = 0; i < m; ++i) {
            len += grid[i][0];
            dp[i][0] = len;
        }
        len = 0;
        for (int j = 0; j < n; ++j) {
            len += grid[0][j];
            dp[0][j] = len;
        }
        for (int i = 1; i < m; ++i) {
            for (int j = 1; j < n; ++j) {
                dp[i][j] = std::min(dp[i - 1][j], dp[i][j - 1]) + grid[i][j];
            }
        }
        return dp[m - 1][n - 1];
    }
};
```