# 剑指 Offer 47. 礼物的最大价值

在一个 m*n 的棋盘的每一格都放有一个礼物，每个礼物都有一定的价值（价值大于 0）。你可以从棋盘的左上角开始拿格子里的礼物，并每次向右或者向下移动一格、直到到达棋盘的右下角。给定一个棋盘及其上面的礼物的价值，请计算你最多能拿到多少价值的礼物？



##示例 1:
```
输入:
[
[1,3,1],
[1,5,1],
[4,2,1]
]
输出: 12
解释: 路径 1→3→5→2→1 可以拿到最多价值的礼物
```

提示：

- 0 < grid.length <= 200
- 0 < grid[0].length <= 200

## 题解

### Rust
```rust
impl Solution {
    pub fn max_value(grid: Vec<Vec<i32>>) -> i32 {
        let (n, m) = (grid.len(), grid.first().map_or(0, Vec::len));
        let mut dp = vec![vec![0; m]; n];
        let mut sum = 0;
        for j in 0..m {
            sum += grid[0][j];
            dp[0][j] = sum;
        }
        sum = 0;
        for i in 0..n {
            sum += grid[i][0];
            dp[i][0] = sum;
        }

        for i in 1..n {
            for j in 1..m {
                dp[i][j] = i32::max(dp[i - 1][j], dp[i][j - 1]) + grid[i][j];
            }
        }

        dp[n - 1][m - 1]
    }
}
```

### Go
```go
package main

import "math"

func maxValue(grid [][]int) int {
	n := len(grid)
	m := len(grid[0])
	dp := make([][]int, n)
	for i := 0; i < len(dp); i++ {
		dp[i] = make([]int, m)
	}
	sum := 0
	for j := 0; j < m; j++ {
		sum += grid[0][j]
		dp[0][j] = sum
	}
	sum = 0
	for i := 0; i < n; i++ {
		sum += grid[i][0]
		dp[i][0] = sum
	}
	for i := 1; i < n; i++ {
		for j := 1; j < m; j++ {
			dp[i][j] = int(math.Max(float64(dp[i-1][j]), float64(dp[i][j-1]))) + grid[i][j]
		}
	}
	return dp[n-1][m-1]
}

```

### JavaScript
```javascript
/**
 * @param {number[][]} grid
 * @return {number}
 */
var maxValue = function(grid) {
        let n = grid.length
        let m = grid[0].length
        let dp = Array(n).fill().map(() => Array(m).fill(0))
        let sum = 0
        for (let j = 0; j < m; ++j) {
            sum += grid[0][j]
            dp[0][j] = sum
        }
        sum = 0
        for (let i = 0; i < n; ++i) {
            sum += grid[i][0]
            dp[i][0] = sum
        }
        for (let i = 1; i < n; ++i) {
            for (let j = 1; j < m; ++j) {
                dp[i][j] = Math.max(dp[i-1][j], dp[i][j-1]) + grid[i][j]
            }
        }
        return dp[n-1][m-1]
    };
```

### Python
```python
class Solution:
    def maxValue(self, grid: List[List[int]]) -> int:
        n = len(grid)
        m = len(grid[0])
        dp = [[0] * m for _ in range(n)]
        total = 0
        for i in range(n):
            total += grid[i][0]
            dp[i][0] = total
        total = 0
        for j in range(m):
            total += grid[0][j]
            dp[0][j] = total
        for i in range(1, n):
            for j in range(1, m):
                dp[i][j] = max(dp[i-1][j], dp[i][j-1]) + grid[i][j]
        return dp[n-1][m-1]
    
```

### C++
```c++
class Solution {
public:
    int maxValue(vector <vector<int>> &grid) {
        int n = grid.size();
        int m = grid[0].size();
        vector<vector<int>> dp(n, vector<int>(m));
        int sum = 0;
        for (int j = 0; j < m; ++j) {
            sum += grid[0][j];
            dp[0][j] = sum;
        }
        sum = 0;
        for (int i = 0; i < n; ++i) {
            sum += grid[i][0];
            dp[i][0] = sum;
        }
        for (int i = 1; i < n; ++i) {
            for (int j = 1; j < m; ++j) {
                dp[i][j] = std::max(dp[i - 1][j], dp[i][j - 1]) + grid[i][j];
            }
        }
        return dp[n - 1][m - 1];
    }
};
```