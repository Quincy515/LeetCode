# 309. 最佳买卖股票时机含冷冻期

给定一个整数数组 `prices`，其中第 `prices[i]` 表示第 `i` 天的股票价格 。

设计一个算法计算出最大利润。在满足以下约束条件下，你可以尽可能地完成更多的交易（多次买卖一支股票）:

卖出股票后，你无法在第二天买入股票 (即冷冻期为 1 天)。
注意：你不能同时参与多笔交易（你必须在再次购买前出售掉之前的股票）。



## 示例 1:
```
输入: prices = [1,2,3,0,2]
输出: 3
解释: 对应的交易状态为: [买入, 卖出, 冷冻期, 买入, 卖出]
```

## 示例 2:
```
输入: prices = [1]
输出: 0
```

## 提示：

- 1 <= prices.length <= 5000
- 0 <= prices[i] <= 1000

## 题解
### Rust
```rust
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.is_empty() {
            return 0;
        }
        let n = prices.len();
        let mut dp = vec![vec![0; 4]; n];
        // dp[i][0] 表示第 i 天持有股票时的利润
        // dp[i][1] 表示第 i 天不持有股票时的利润（当天刚卖掉）
        // dp[i][2] 表示第 i 天不持有股票时的利润（冷冻期），昨天刚卖了股票
        // dp[i][3] 表示第 i 天不持有股票时的利润（非冷冻期），昨天也没有持有
        dp[0][0] = -prices[0];
        dp[0][1] = 0;
        dp[0][2] = 0;
        dp[0][3] = 0;
        for i in 1..n {
            dp[i][0] = dp[i - 1][0]
                .max(dp[i - 1][2] - prices[i])
                .max(dp[i - 1][3] - prices[i]);
            dp[i][1] = dp[i - 1][0] + prices[i];
            dp[i][2] = dp[i - 1][1];
            dp[i][3] = i32::max(dp[i - 1][2], dp[i - 1][3]);
        }
        dp[n - 1][0]
            .max(dp[n - 1][1])
            .max(dp[n - 1][2])
            .max(dp[n - 1][3])
    }
}
```

### Go
```go
package main

import "math"

func maxProfit(prices []int) int {
	if len(prices) == 0 {
		return 0
	}
	n := len(prices)
	dp := make([][]int, n)
	for i := 0; i < len(dp); i++ {
		dp[i] = make([]int, 4)
	}
	dp[0][0] = -prices[0]
	dp[0][1] = 0
	dp[0][2] = 0
	dp[0][3] = 0
	for i := 1; i < n; i++ {
		dp[i][0] = max3(dp[i-1][0], dp[i-1][2]-prices[i], dp[i-1][3]-prices[i])
		dp[i][1] = dp[i-1][0] + prices[i]
		dp[i][2] = dp[i-1][1]
		dp[i][3] = int(math.Max(float64(dp[i-1][2]), float64(dp[i-1][3])))
	}
	return max4(dp[n-1][0], dp[n-1][1], dp[n-1][2], dp[n-1][3])
}
func max3(a, b, c int) int {
	max := a
	if b > max {
		max = b
	}
	if c > max {
		max = c
	}
	return max
}
func max4(a, b, c, d int) int {
	max := a
	if b > max {
		max = b
	}
	if c > max {
		max = c
	}
	if d > max {
		max = d
	}
	return max
}

```

### JavaScript
```javascript
/**
 * @param {number[]} prices
 * @return {number}
 */
var maxProfit = function(prices) {
    let n = prices.length
    if (n == 0) {
        return 0
    }
    let dp = Array(n).fill().map(() => Array(4).fill(0))
    dp [0][0] = -prices[0]
    dp[0][1] = 0
    dp[0][2] = 0
    dp[0][3] = 0
    for (let i = 1; i < n; ++i) {
        dp[i][0] = Math.max(dp[i-1][0], dp[i-1][2]-prices[i], dp[i-1][3]-prices[i])
        dp[i][1] = dp[i-1][0]+prices[i]
        dp[i][2] = dp[i-1][1]
        dp[i][3] = Math.max(dp[i-1][2], dp[i-1][3])
    }
    return Math.max(...dp[n-1])
};
```

### Python
```python
class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        if len(prices) == 0:
            return 0
        n = len(prices)
        dp = [[0,0,0,0] for _ in range(n)]
        dp[0][0] = -prices[0]
        dp[0][1] = 0
        dp[0][2] = 0
        dp[0][3] = 0
        for i in range(1,n):
            dp[i][0] = max(dp[i-1][0], dp[i-1][2] - prices[i], dp[i-1][3] - prices[i])
            dp[i][1] = dp[i-1][0] + prices[i]
            dp[i][2] = dp[i-1][1]
            dp[i][3] = max(dp[i-1][2], dp[i-1][3])
        return max(dp[n-1][0], dp[n-1][1],dp[n-1][2],dp[n-1][3])
```

### C++
```c++
class Solution {
public:
    int maxProfit(vector<int>& prices) {
        if (prices.size() == 0) return 0;
        int n = prices.size();
        vector<vector<int>> dp(n, vector<int>(4));
        dp[0][0] = -prices[0];
        dp[0][1] = 0;
        dp[0][2] = 0;
        dp[0][3] = 0;
        for (int i = 1; i < n; ++i) {
            dp[i][0] = max3(dp[i-1][0], dp[i-1][2] - prices[i], dp[i - 1][3] -
                                                                prices[i]);
            dp[i][1] = dp[i - 1][0] + prices[i];
            dp[i][2] = dp[i - 1][1];
            dp[i][3] = std::max(dp[i-1][2], dp[i-1][3]);
        }
        return max4(dp[n-1][0], dp[n-1][1], dp[n-1][2], dp[n-1][3]);
    }
    int max3(int a, int b, int c) {
        return std::max(std::max(a, b), c);
    }
    int max4(int a, int b, int c, int d) {
        return std::max(std::max(std::max(a, b), c), d);
    }
};
```