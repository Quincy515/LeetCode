# 剑指 Offer 14- I. 剪绳子
给你一根长度为 n 的绳子，请把绳子剪成整数长度的 m 段（m、n都是整数，n>1并且m>1），每段绳子的长度记为 k[0],k[1]...k[m-1] 。请问 k[0]*k[1]*...*k[m-1] 可能的最大乘积是多少？例如，当绳子的长度是8时，我们把它剪成长度分别为2、3、3的三段，此时得到的最大乘积是18。

## 示例 1：
```
输入: 2
输出: 1
解释: 2 = 1 + 1, 1 × 1 = 1
```

## 示例 2:

```
输入: 10
输出: 36
解释: 10 = 3 + 3 + 4, 3 × 3 × 4 = 36
```

## 提示：

- 2 <= n <= 58

## 题解
### Rust
```rust
impl Solution {
    // 递归转非递归的写法，类比上台阶
    pub fn cutting_rope(n: i32) -> i32 {
        let n = n as usize;
        if n == 1 {
            return 1;
        }
        if n == 2 {
            return 1;
        }
        if n == 3 {
            return 2;
        }
        // dp[i]表示长度为 i 的最大乘积
        let mut dp: Vec<i32> = vec![0; n + 1];
        dp[0] = 1;
        for i in 1..=n {
            for j in 1..=i {
                if dp[i] < j as i32 * dp[i - j] {
                    dp[i] = j as i32 * dp[i - j];
                }
            }
        }
        dp[n]
    }
}
```

### Go
```go
package main

func cuttingRope(n int) int {
	if n == 1 {
		return 1
	}
	if n == 2 {
		return 1
	}
	if n == 3 {
		return 2
	}
	dp := make([]int, n+1)
	dp[0] = 1
	for i := 1; i <= n; i++ {
		for j := 1; j <= i; j++ {
			if dp[i] < j*dp[i-j] {
				dp[i] = j * dp[i-j]
			}
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
var cuttingRope = function(n) {
    if (n == 1) return 1
    if (n == 2) return 1
    if (n == 3) return 2
    let dp = new Array(n+1).fill(0)
    dp[0] = 1
    for (let i = 1; i <= n; ++i) {
        for (let j = 1; j <= i; j++) {
            if (dp[i] < j*dp[i-j]) {
                dp[i] = j*dp[i-j]
            }
        }
    }
    return dp[n]
};
```

### Python
```python
class Solution:
    def cuttingRope(self, n: int) -> int:
        if n == 1:
            return 1
        if n == 2:
            return 1
        if n == 3:
            return 2
        dp = [0] * (n +1)
        dp[0] = 1
        for i in range(1,n+1):
            for j in range(1,i+1):
                if dp[i] < dp[i-j] * j:
                   dp[i] = j * dp[i-j]
        return dp[n]
```

### C++
```c++
class Solution {
public:
    int cuttingRope(int n) {
        if (n == 1) return 1;
        if (n == 2) return 1;
        if (n == 3) return 2;
        std::vector<int> dp(n + 1);
        dp[0] = 1;
        for (int i = 1; i <= n; ++i) {
            for (int j = 1; j <= i; ++j) {
                if (dp[i] < j * dp[i - j]) {
                    dp[i] = j * dp[i - j];
                }
            }
        }
        return dp[n];
    }
};
```