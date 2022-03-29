# 120. 三角形最小路径和
给定一个三角形，找出自顶向下的最小路径和。每一步只能移动到下一行中相邻的结点上。

例如，给定三角形：

<pre>
[
     [<strong>2</strong>],
    [<strong>3</strong>,4],
   [6,<strong>5</strong>,7],
  [4,<strong>1</strong>,8,3]
]
</pre>

自顶向下的最小路径和为 ```11```（即，**2** + **3** + **5** + **1** = 11）。

#### 说明:
如果你可以只使用 *O*(*n*) 的额外空间（*n* 为三角形的总行数）来解决这个问题，那么你的算法会很加分。

## 题解

### Rust
```rust
impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let n = triangle.len();
        let mut dp = vec![vec![0; n]; n];
        dp[0][0] = triangle[0][0];
        for i in 1..n {
            dp[i][0] = dp[i - 1][0] + triangle[i][0];
            for j in 1..i {
                dp[i][j] = i32::min(dp[i - 1][j], dp[i - 1][j - 1]) + triangle[i][j];
            }
            dp[i][i] = dp[i - 1][i - 1] + triangle[i][i];
        }

        let mut res = i32::MAX;
        for j in 0..n {
            if dp[n - 1][j] < res {
                res = dp[n - 1][j];
            }
        }
        res
    }
}
```

### Go
```go
package main

import "math"

func minimumTotal(triangle [][]int) int {
	n := len(triangle)
	dp := make([][]int, n)
	for i := 0; i < len(dp); i++ {
		dp[i] = make([]int, n)
	}
	dp[0][0] = triangle[0][0]
	for i := 1; i < n; i++ {
		dp[i][0] = dp[i-1][0] + triangle[i][0]
		for j := 1; j < i; j++ {
			dp[i][j] = int(math.Min(float64(dp[i-1][j]), float64(dp[i-1][j-1]))) + triangle[i][j]
		}
		dp[i][i] = dp[i-1][i-1] + triangle[i][i]
	}
	res := math.MaxInt32
	for j := 0; j < n; j++ {
		if dp[n-1][j] < res {
			res = dp[n-1][j]
		}
	}
	return res
}

```

### JavaScript
```javascript
/**
 * @param {number[][]} triangle
 * @return {number}
 */
var minimumTotal = function(triangle) {
    let n = triangle.length
    let dp = Array(n).fill().map(() => Array(n).fill(0))
    dp[0][0] = triangle[0][0]
    for (let i = 1; i < n; ++i) {
        dp[i][0] = dp[i-1][0] + triangle[i][0]
        for (let j = 1; j < i; ++j) {
            dp[i][j] = Math.min(dp[i-1][j], dp[i-1][j-1]) + triangle[i][j]
        }
        dp[i][i] = dp[i-1][i-1] + triangle[i][i]
    }
    let res = Number.MAX_SAFE_INTEGER
    for (let j = 0; j < n; ++j) {
        if (dp[n-1][j] < res) {
            res = dp[n-1][j]
        }
    }
    return res
};
```

### Python
```python
class Solution:
    def minimumTotal(self, triangle: List[List[int]]) -> int:
        n = len(triangle)
        dp = [[float("inf")] * n for _ in range(n)]
        dp[0][0] = triangle[0][0]
        for i in range(1,n):
            dp[i][0] = dp[i-1][0] + triangle[i][0]
            for j in range(1,i):
                dp[i][j] = min(dp[i-1][j],dp[i-1][j-1]) + triangle[i][j]
                dp[i][i] = dp[i-1][i-1] + triangle[i][i]
        res = float("inf")
        for j in range(n):
            if dp[n-1][j] < res:
                res = dp[n-1][j]
        return res

```

### C++
```c++
class Solution {
public:
    int minimumTotal(vector <vector<int>> &triangle) {
        int n = triangle.size();
        vector<vector<int>> dp(n, vector<int>(n));
        dp[0][0] = triangle[0][0];
        for (int i = 1; i < n; ++i) {
            dp[i][0] = dp[i - 1][0] + triangle[i][0];
            for (int j = 1; j < i; ++j) {
                dp[i][j] = std::min(dp[i - 1][j], dp[i - 1][j - 1]) + triangle[i][j];
            }
            dp[i][i] = dp[i - 1][i - 1] + triangle[i][i];
        }
        int res = INT_MAX;
        for (int j = 0; j < n; ++j) {
            if (dp[n - 1][j] < res) res = dp[n - 1][j];
        }
        return res;
    }
};
```