# 322. 零钱兑换
给定不同面额的硬币 coins 和一个总金额 amount。编写一个函数来计算可以凑成总金额所需的最少的硬币个数。如果没有任何一种硬币组合能组成总金额，返回 ```-1```。

#### 示例 1:
<pre>
<strong>输入:</strong> coins = [1, 2, 5], amount = 11
<strong>输出:</strong> 3
<strong>解释:</strong> 11 = 5 + 5 + 1
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> coins = [2], amount = 3
<strong>输出:</strong> -1
</pre>

#### 说明:
你可以认为每种硬币的数量是无限的。

## 题解

### Rust
```rust
impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let n = coins.len();
        // 第 i 个硬币决策完之后，凑足金额 j 需要的最少硬币数 dp[i][j]
        let mut dp = vec![vec![i32::MAX; amount as usize + 1]; n];
        let mut c = 0;
        while c <= amount / coins[0] {
            dp[0][(c * coins[0]) as usize] = c;
            c += 1;
        }
        for i in 1..n {
            for j in 0..=amount as usize {
                let k = j / coins[i] as usize;
                for c in 0..=k {
                    if dp[i - 1][j - c * coins[i] as usize] != i32::MAX
                        && dp[i - 1][j - c * coins[i] as usize] + (c as i32) < dp[i][j]
                    {
                        dp[i][j] = dp[i - 1][j - c * coins[i] as usize] + c as i32;
                    }
                }
            }
        }
        if dp[n - 1][amount as usize] == i32::MAX {
            return -1;
        }
        dp[n - 1][amount as usize]
    }

    pub fn coin_change2(coins: Vec<i32>, amount: i32) -> i32 {
        let k = coins.len();
        let mut dp = vec![i32::MAX; amount as usize + 1];
        dp[0] = 0;
        for i in 1..=amount {
            for j in 0..k {
                if i - coins[j] >= 0
                    && dp[(i - coins[j]) as usize] != i32::MAX
                    && dp[(i - coins[j]) as usize] + 1 < dp[i as usize]
                {
                    dp[i as usize] = dp[(i - coins[j]) as usize] + 1;
                }
            }
        }
        if dp[amount as usize] == i32::MAX {
            return -1;
        }
        dp[amount as usize]
    }
}
```

### Go
```go
package main

import "math"

func coinChange(coins []int, amount int) int {
	n := len(coins)
	dp := make([][]int, n)
	for i := 0; i < n; i++ {
		dp[i] = make([]int, amount+1)
		for j := 0; j <= amount; j++ {
			dp[i][j] = math.MaxInt32
		}
	}
	for c := 0; c <= amount/coins[0]; c++ {
		dp[0][c*coins[0]] = c
	}
	for i := 1; i < n; i++ {
		for j := 0; j <= amount; j++ {
			k := j / coins[i]
			for c := 0; c <= k; c++ {
				if dp[i-1][j-c*coins[i]] != math.MinInt32 && dp[i-1][j-c*coins[i]]+c < dp[i][j] {
					dp[i][j] = dp[i-1][j-c*coins[i]] + c
				}
			}
		}
	}
	if dp[n-1][amount] == math.MaxInt32 {
		return -1
	}
	return dp[n-1][amount]
}

func coinChange(coins []int, amount int) int {
	k := len(coins)
	dp := make([]int, amount+1)
	for i := 0; i <= amount; i++ {
		dp[i] = math.MaxInt32
	}
	dp[0] = 0
	for i := 1; i <= amount; i++ {
		for j := 0; j < k; j++ {
			if i-coins[j] >= 0 && dp[i-coins[j]] != math.MaxInt32 && dp[i-coins[j]]+1 < dp[i] {
				dp[i] = dp[i-coins[j]] + 1
			}
		}
	}
	if dp[amount] == math.MaxInt32 {
		return -1
	}
	return dp[amount]
}
```

### JavaScript
```javascript
/**
 * @param {number[]} coins
 * @param {number} amount
 * @return {number}
 */
var coinChange = function(coins, amount) {
    let n = coins.length
    let dp = Array(n).fill().map(() => Array(amount+1).fill(Number.MAX_SAFE_INTEGER))
    for (let c = 0; c <= Math.floor(amount/coins[0]); ++c) {
        dp[0][c*coins[0]] = c
    }
    for (let i = 1; i < n; ++i) {
        for (let j = 0; j <= amount; ++j) {
            let k = Math.floor(j/coins[i])
            for (let c = 0; c <= k; ++c) {
                if (dp[i-1][j-c*coins[i]] != Number.MAX_SAFE_INTEGER &&
                    dp[i-1][j-c*coins[i]] + c < dp[i][j]) {
                    dp[i][j] = dp[i-1][j-c*coins[i]] + c
                }
            }
        }
    }
    if (dp[n-1][amount] == Number.MAX_SAFE_INTEGER) {
        return -1
    }
    return dp[n-1][amount]
};


/**
 * @param {number[]} coins
 * @param {number} amount
 * @return {number}
 */
var coinChange = function(coins, amount) {
    let k = coins.length
    let dp = Array(amount+1).fill(Number.MAX_SAFE_INTEGER)
    dp[0] = 0
    for (let i = 1; i <= amount; ++i) {
        for (let j = 0; j < k; ++j) {
            if (i-coins[j] >= 0 &&
                dp[i-coins[j]] != Number.MAX_SAFE_INTEGER &&
                dp[i-coins[j]] + 1 < dp[i]) {
                dp[i] = dp[i-coins[j]] + 1
            }
        }
    }
    if (dp[amount] == Number.MAX_SAFE_INTEGER) {
        return -1
    }
    return dp[amount]
};
```

### Python
```python
class Solution:
    def coinChange(self, coins: List[int], amount: int) -> int:
        n = len(coins)
        dp = [[float('inf')] * (amount+1) for i in range(n)]
        for c in range(amount//coins[0] + 1):
            dp[0][c * coins[0]] = c

        for i in range(1, n):
            for j in range(amount+1):
                k = j // coins[i]
                for c in range(k+1):
                    if dp[i-1][j - c * coins[i]] != float('inf') and dp[i-1][j - c * coins[i]] + c < dp[i][j]:
                        dp[i][j] = dp[i-1][j - c * coins[i]] + c
                        
        if dp[n-1][amount] == float('inf'):
            return -1
        return dp[n-1][amount]

class Solution:
    def coinChange(self, coins: List[int], amount: int) -> int:
        k = len(coins)
        dp = [float("inf")] * (amount+1)
        dp[0] = 0
        for i in range(1, amount+1):
            for j in range(k):
                if i - coins[j] >=0 and dp[i- coins[j]] != float("inf") and dp[i-coins[j]] + 1 < dp[i]:
                    dp[i] = dp[i - coins[j]] + 1
        if dp[amount] == float("inf"):
            return -1
        return dp[amount]
```

### C++
```c++
class Solution {
public:
    int coinChange(vector<int> &coins, int amount) {
        int n = coins.size();
        vector <vector<int>> dp(n, vector<int>(amount + 1));
        for (int i = 0; i < n; ++i) {
            for (int j = 0; j <= amount; ++j) {
                dp[i][j] = INT_MAX;
            }
        }
        for (int c = 0; c <= amount / coins[0]; ++c) {
            dp[0][c * coins[0]] = c;
        }
        for (int i = 1; i < n; ++i) {
            for (int j = 0; j <= amount; ++j) {
                int k = j / coins[i];
                for (int c = 0; c <= k; ++c) {
                    if (dp[i - 1][j - c * coins[i]] != INT_MAX &&
                        dp[i - 1][j - c * coins[i]] + c < dp[i][j]) {
                        dp[i][j] = dp[i - 1][j - c * coins[i]] + c;
                    }
                }
            }
        }
        if (dp[n - 1][amount] == INT_MAX) return -1;
        return dp[n - 1][amount];
    }
};


class Solution {
public:
    int coinChange(vector<int>& coins, int amount) {
        int k = coins.size();
        vector<int> dp(amount + 1);
        for (int i = 0; i <= amount; ++i) {
            dp[i] = INT_MAX;
        }
        dp[0] = 0;
        for (int i = 1; i <= amount; ++i) {
            for (int j = 0; j < k; ++j) {
                if (i - coins[j] >= 0 &&
                    dp[i - coins[j]] != INT_MAX &&
                    dp[i - coins[j]] + 1 < dp[i]) {
                    dp[i] = dp[i - coins[j]] + 1;
                }
            }
        }
        if (dp[amount] == INT_MAX) return -1;
        return dp[amount];
    }
};
```