# 72. 编辑距离

给你两个单词 word1 和 word2， 请返回将 word1 转换成 word2 所使用的最少操作数  。

你可以对一个单词进行如下三种操作：

- 插入一个字符
- 删除一个字符
- 替换一个字符


## 示例 1：
```
输入：word1 = "horse", word2 = "ros"
输出：3
解释：
horse -> rorse (将 'h' 替换为 'r')
rorse -> rose (删除 'r')
rose -> ros (删除 'e')
```

## 示例 2：
```
输入：word1 = "intention", word2 = "execution"
输出：5
解释：
intention -> inention (删除 't')
inention -> enention (将 'i' 替换为 'e')
enention -> exention (将 'n' 替换为 'x')
exention -> exection (将 'n' 替换为 'c')
exection -> execution (插入 'u')
```

## 提示：

- 0 <= word1.length, word2.length <= 500
- word1 和 word2 由小写英文字母组成

## 题解
### Rust
```rust
impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let (n, m) = (word1.len(), word2.len());
        if n == 0 {
            return m as i32;
        }
        if m == 0 {
            return n as i32;
        }
        let (w1, w2) = (word1.as_bytes(), word2.as_bytes());
        //dp[i][j] 表示 w1[0~i-1]（长度为 i 子串）和 w2[0~j-1]（长度为 j 子串）的最小编辑距离
        let mut dp = vec![vec![0; m + 1]; n + 1];
        for j in 0..=m {
            dp[0][j] = j as i32;
        }
        for i in 0..=n {
            dp[i][0] = i as i32;
        }
        for i in 1..=n {
            for j in 1..=m {
                if w1[i - 1] == w2[j - 1] {
                    dp[i][j] = (dp[i - 1][j] + 1)
                        .min(dp[i][j - 1] + 1)
                        .min(dp[i - 1][j - 1]);
                } else {
                    dp[i][j] = (dp[i - 1][j] + 1)
                        .min(dp[i][j - 1] + 1)
                        .min(dp[i - 1][j - 1] + 1)
                }
            }
        }

        dp[n][m]
    }
}

```

### Go
```go
package main

import "math"

func minDistance(word1 string, word2 string) int {
	n := len(word1)
	m := len(word2)
	if n == 0 {
		return m
	}
	if m == 0 {
		return n
	}
	w1 := []byte(word1)
	w2 := []byte(word2)
	dp := make([][]int, n+1)
	for i := 0; i < len(dp); i++ {
		dp[i] = make([]int, m+1)
	}
	for j := 0; j <= m; j++ {
		dp[0][j] = j
	}
	for i := 0; i <= n; i++ {
		dp[i][0] = i
	}
	for i := 1; i <= n; i++ {
		for j := 1; j <= m; j++ {
			if w1[i-1] == w2[j-1] {
				dp[i][j] = min3(dp[i-1][j]+1, dp[i][j-1]+1, dp[i-1][j-1])
			} else {
				dp[i][j] = min3(dp[i-1][j]+1, dp[i][j-1]+1, dp[i-1][j-1]+1)
			}
		}
	}
	return dp[n][m]
}

func min3(n1, n2, n3 int) int { return int(math.Min(float64(n1), math.Min(float64(n2), float64(n3)))) }

```

### JavaScript
```javascript
/**
 * @param {string} word1
 * @param {string} word2
 * @return {number}
 */
var minDistance = function(word1, word2) {
    let n = word1.length
    let m = word2.length
    if (n == 0) return m
    if (m == 0) return n
    let dp = new Array(n+1).fill().map(() => new Array(m+1).fill(0))
    for (let j = 0; j <= m; j++) {
        dp[0][j] = j
    }
    for (let i = 0; i <= n; i++) {
        dp[i][0] = i
    }
    for (let i = 1; i <= n; i++) {
        for (let j = 1; j <= m; j++) {
            if (word1[i-1] == word2[j-1]) {
                dp[i][j] = Math.min(dp[i-1][j]+1, dp[i][j-1]+1, dp[i-1][j-1])
            } else {
                dp[i][j] = Math.min(dp[i-1][j]+1, dp[i][j-1]+1, dp[i-1][j-1]+1)
            }
        }
    }
    return dp[n][m]
};
```

### Python
```python
class Solution:
    def minDistance(self, word1: str, word2: str) -> int:
        n = len(word1)
        m = len(word2)
        dp = [[0] * (m+1) for _ in range(n+1)]
        for j in range(m+1):
            dp[0][j] = j
        for i in range(n+1):
            dp[i][0] = i
        for i in range(1,n+1):
            for j in range(1,m+1):
                if word1[i-1] == word2[j-1]:
                    dp[i][j] = min(dp[i-1][j]+1, dp[i][j-1]+1, dp[i-1][j-1])
                else:
                    dp[i][j] = min(dp[i-1][j]+1, dp[i][j-1]+1, dp[i-1][j-1]+1)
        return dp[n][m]
```

### C++
```c++
class Solution {
public:
    int minDistance(string word1, string word2) {
        int n = word1.size();
        int m = word2.size();
        if (n == 0) return m;
        if (m == 0) return n;
        vector<vector<int>> dp(n + 1, vector<int>(m + 1));
        for (int j = 0; j <= m; ++j) {
            dp[0][j] = j;
        }
        for (int i = 0; i <= n; ++i) {
            dp[i][0] = i;
        }
        for (int i = 1; i <= n; ++i) {
            for (int j = 1; j <= m; ++j) {
                if (word1[i - 1] == word2[j - 1]) {
                    dp[i][j] = min3(dp[i-1][j] + 1, dp[i][j-1] + 1, dp[i - 1][j - 1]);
                } else {
                    dp[i][j] = min3(dp[i-1][j] + 1, dp[i][j-1] + 1, dp[i - 1][j - 1] + 1);
                }
            }
        }
        return dp[n][m];
    }
    int min3(int n1, int n2, int n3) {
        return std::min(n1, std::min(n2, n3));
    }
};
```