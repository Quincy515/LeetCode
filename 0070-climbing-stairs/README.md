# 70. 爬楼梯
假设你正在爬楼梯。需要 *n* 阶你才能到达楼顶。

每次你可以爬 1 或 2 个台阶。你有多少种不同的方法可以爬到楼顶呢？

**注意:** 给定 *n* 是一个正整数。

#### 示例 1:
<pre>
<strong>输入:</strong> 2
<strong>输出:</strong> 2
<strong>解释:</strong> 有两种方法可以爬到楼顶。
1.  1 阶 + 1 阶
2.  2 阶
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> 3
<strong>输出:</strong> 3
<strong>解释:</strong> 有三种方法可以爬到楼顶。
1.  1 阶 + 1 阶 + 1 阶
2.  1 阶 + 2 阶
3.  2 阶 + 1 阶
</pre>

## 题解 
### Rust
```rust
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let n = n as usize;
        if n <= 2 {
            return n as i32;
        }

        let mut dp = vec![0; n + 1];
        dp[1] = 1;
        dp[2] = 2;
        for i in 3..=n {
            dp[i] = dp[i - 1] + dp[i - 2];
        }
        dp[n]
    }

    pub fn climb_stairs2(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![0; n + 1];
        dp[0] = 1;
        for i in 1..=n {
            if i >= 1 {
                dp[i] += dp[i - 1];
            }
            if i >= 2 {
                dp[i] += dp[i - 2];
            }
        }
        dp[n]
    }
}
```

### Go
```go
package main

func climbStairs(n int) int {
	if n <= 2 {
		return n
	}
	dp := make([]int, n+1)
	dp[1] = 1
	dp[2] = 2
	for i := 3; i <= n; i++ {
		dp[i] = dp[i-1] + dp[i-2]
	}
	return dp[n]
}
func climbStairs(n int) int {
	dp := make([]int, n+1)
	dp[0] = 1
	for i := 1; i <= n; i++ {
		if i-1 >= 0 {
			dp[i] += dp[i-1]
		}
		if i-2 >= 0 {
			dp[i] += dp[i-2]
		}
	}
	return dp[n]
}
```

### JavaScript
```javascript
/**
 * @param {number} n
 * @return {number}
 */
var climbStairs = function(n) {
    if (n <= 2) {
        return n
    }
    let dp = new Array(n+1)
    dp[1] = 1
    dp[2] = 2
    for (let i = 3; i <= n; ++i) {
        dp[i] = dp[i-1] + dp[i-2]
    }
    return dp[n]
};

/**
 * @param {number} n
 * @return {number}
 */
var climbStairs = function(n) {
    let dp = Array(n+1).fill(0)
    dp[0] = 1
    for (let i = 1; i <= n; ++i) {
        if (i-1 >= 0) {
            dp[i] += dp[i-1]
        }
        if (i-2 >= 0) {
            dp[i] += dp[i-2]
        }
    }
    return dp[n]
};
```

### Python
```python
class Solution:
    def climbStairs(self, n: int) -> int:
        if n <= 2:
            return n
        dp = [0] * (n + 1)
        dp[1] = 1
        dp[2] = 2
        for i in range(3, n+1):
            dp[i] = dp[i-1] + dp[i-2]
        return dp[n]

class Solution:
    def climbStairs(self, n: int) -> int:
        dp = [0] * (n + 1)
        dp[0] = 1
        for i in range(1, n+1):
            if i - 1 >= 0:
                dp[i] += dp[i-1]
            if i - 2 >= 0:
                dp[i] += dp[i-2]
        return dp[n]

```

### C++
```c++
class Solution {
public:
    int climbStairs(int n) {
        if (n <= 2) return n;
        std::vector<int> dp(n+1);
        dp[1] = 1;
        dp[2] = 2;
        for (int i = 3; i <= n; ++i) {
            dp[i] = dp[i-1] + dp[i-2];
        }
        return dp[n];
    }
};

class Solution {
public:
    int climbStairs(int n) {
        std::vector<int> dp(n+1);
        dp[0] = 1;
        for (int i = 1; i <= n; ++i) {
            if (i - 1 >= 0) dp[i] += dp[i - 1];
            if (i - 2 >= 0) dp[i] += dp[i - 2];
        }
        return dp[n];
    }
};
```