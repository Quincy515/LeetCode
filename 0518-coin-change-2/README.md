# 518. 零钱兑换 II
给定不同面额的硬币和一个总金额。写出函数来计算可以凑成总金额的硬币组合数。假设每一种面额的硬币有无限个。

#### 示例 1:
<pre>
<strong>输入:</strong> amount = 5, coins = [1, 2, 5]
<strong>输出:</strong> 4
<strong>解释:</strong> 有四种方式可以凑成总金额:
5=5
5=2+2+1
5=2+1+1+1
5=1+1+1+1+1
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> amount = 3, coins = [2]
<strong>输出:</strong> 0
<strong>解释:</strong> 只用面额2的硬币不能凑成总金额3。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> amount = 10, coins = [10]
<strong>输出:</strong> 1
</pre>

#### 注意:
你可以假设：
* 0 <= amount (总金额) <= 5000
* 1 <= coin (硬币面额) <= 5000
* 硬币种类不超过 500 种
* 结果符合 32 位符号整数

## 题解

### Rust
```rust
impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        let n = coins.len();
        let mut dp = vec![vec![0; amount as usize + 1]; n];
        for c in 0..=amount / coins[0] {
            dp[0][(c * coins[0]) as usize] = 1;
        }
        for i in 1..n {
            for j in 0..=amount as usize {
                let k = j / coins[i] as usize;
                for c in 0..=k {
                    dp[i][j] += dp[i - 1][j - c * coins[i] as usize];
                }
            }
        }

        dp[n - 1][amount as usize]
    }
}
```

### Go
```go
package main

func change(amount int, coins []int) int {
	n := len(coins)
	dp := make([][]int, n)
	for i := 0; i < len(dp); i++ {
		dp[i] = make([]int, amount+1)
	}
	for c := 0; c <= amount/coins[0]; c++ {
		dp[0][c*coins[0]] = 1
	}
	for i := 1; i < n; i++ {
		for j := 0; j <= amount; j++ {
			k := j / coins[i]
			for c := 0; c <= k; c++ {
				dp[i][j] += dp[i-1][j-c*coins[i]]
			}
		}
	}
	return dp[n-1][amount]
}

```

### JavaScript
```javascript
/**
 * @param {number} amount
 * @param {number[]} coins
 * @return {number}
 */
var change = function(amount, coins) {
    let n = coins.length
    let dp = Array(n).fill().map(() => Array(amount+1).fill(0))
    for (let c = 0; c <= Math.floor(amount/coins[0]); ++c) {
        dp[0][c*coins[0]] = 1
    }
    for (let i = 1; i < n; ++i) {
        for (let j = 0; j <= amount; ++j) {
            let k = Math.floor(j/coins[i])
            for (let c = 0; c <= k; ++c) {
                dp[i][j] += dp[i-1][j-c*coins[i]]
            }
        }
    }
    return dp[n-1][amount]
};
```

### Python
```python
class Solution:
    def change(self, amount: int, coins: List[int]) -> int:
        n = len(coins)
        dp = [[0] * (amount + 1) for _ in range(n)]
        for c in range(amount // coins[0] + 1):
            dp[0][c*coins[0]] = 1
        for i in range(1, n):
            for j in range(amount+1):
                k = j // coins[i] + 1
                for c in range(k):
                    dp[i][j] += dp[i-1][j - c * coins[i]]
        return dp[n-1][amount]

```
### C++
```c++
class Solution {
public:
    int change(int amount, vector<int> &coins) {
        int n = coins.size();
        vector<vector<int>> dp(n, vector<int>(amount + 1));
        for (int c = 0; c <= amount / coins[0]; ++c) {
            dp[0][c * coins[0]] = 1;
        }
        for (int i = 1; i < n; ++i) {
            for (int j = 0; j <= amount; ++j) {
                int k = j / coins[i];
                for (int c = 0; c <= k; ++c) {
                    dp[i][j] += dp[i - 1][j - c * coins[i]];
                }
            }
        }
        return dp[n - 1][amount];
    }
};
```