# 1143. 最长公共子序列

给定两个字符串 `text1` 和 `text2`，返回这两个字符串的最长 公共子序列 的长度。如果不存在 公共子序列 ，返回 0 。

一个字符串的 **子序列** 是指这样一个新的字符串：它是由原字符串在不改变字符的相对顺序的情况下删除某些字符（也可以不删除任何字符）后组成的新字符串。

- 例如，"ace" 是 "abcde" 的子序列，但 "aec" 不是 "abcde" 的子序列。

两个字符串的 公共子序列 是这两个字符串所共同拥有的子序列。


## 示例 1：
```
输入：text1 = "abcde", text2 = "ace"
输出：3  
解释：最长公共子序列是 "ace" ，它的长度为 3 。
```

## 示例 2：
```
输入：text1 = "abc", text2 = "abc"
输出：3
解释：最长公共子序列是 "abc" ，它的长度为 3 。
```

## 示例 3：
```
输入：text1 = "abc", text2 = "def"
输出：0
解释：两个字符串没有公共子序列，返回 0 。
```

## 提示：

- 1 <= text1.length, text2.length <= 1000
- text1 和 text2 仅由小写英文字符组成。

## 题解
### Rust
```rust
impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let (n, m) = (text1.len(), text2.len());
        let (t1, t2) = (text1.as_bytes(), text2.as_bytes());

        // dp[i][j] 表示 text1[0~i-1] （长度为 i 子串）和 text2[0~j-1]（长度 j 的字串）的最长公共子序列（LCS）
        let mut dp = vec![vec![0; m + 1]; n + 1];
        for j in 0..=m {
            dp[0][j] = 0;
        }
        for i in 0..=n {
            dp[i][0] = 0;
        }
        for i in 1..=n {
            for j in 1..=m {
                if t1[i - 1] == t2[j - 1] {
                    dp[i][j] = (dp[i - 1][j - 1] + 1).max(dp[i - 1][j]).max(dp[i][j - 1])
                } else {
                    dp[i][j] = dp[i - 1][j - 1].max(dp[i - 1][j]).max(dp[i][j - 1]);
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

func longestCommonSubsequence(text1 string, text2 string) int {
	n := len(text1)
	m := len(text2)
	t1 := []byte(text1)
	t2 := []byte(text2)
	dp := make([][]int, n+1)
	for i := 0; i < len(dp); i++ {
		dp[i] = make([]int, m+1)
	}
	for j := 0; j <= m; j++ {
		dp[0][j] = 0
	}
	for i := 0; i <= n; i++ {
		dp[i][0] = 0
	}
	for i := 1; i <= n; i++ {
		for j := 1; j <= m; j++ {
			if t1[i-1] == t2[j-1] {
				dp[i][j] = max3(dp[i-1][j-1]+1, dp[i-1][j], dp[i][j-1])
			} else {
				dp[i][j] = max3(dp[i-1][j-1], dp[i-1][j], dp[i][j-1])
			}
		}
	}
	return dp[n][m]
}

func max3(a, b, c int) int {
	maxv := a
	if maxv < b {
		maxv = b
	}
	if maxv < c {
		maxv = c
	}
	return maxv
}

```

### JavaScript
```javascript
/**
 * @param {string} text1
 * @param {string} text2
 * @return {number}
 */
var longestCommonSubsequence = function(text1, text2) {
    let n = text1.length
    let m = text2.length
    let dp = new Array(n+1).fill().map(() => new Array(m+1).fill(0))
    for (let i = 1; i <= n; ++i) {
        for (let j = 1; j <= m; ++j) {
            if (text1[i-1] == text2[j-1]) {
                dp[i][j] = Math.max(dp[i-1][j-1]+1, dp[i-1][j], dp[i][j-1])
            } else {
                dp[i][j] = Math.max(dp[i-1][j-1], dp[i-1][j], dp[i][j-1])
            }
        }
    }
    return dp[n][m]
};
```

### Python
```python
class Solution:
    def longestCommonSubsequence(self, text1: str, text2: str) -> int:
        n = len(text1)
        m = len(text2)
        dp = [[0] * (m+1) for _ in range(n+1)]
        for j in range(m+1):
            dp[0][j] = 0
        for i in range(n+1):
            dp[i][0] = 0
        for i in range(1,n+1):
            for j in range(1,m+1):
                if text1[i-1] == text2[j-1]:
                    dp[i][j] = max(dp[i-1][j], dp[i][j-1], dp[i-1][j-1] + 1)
                else:
                    dp[i][j] = max(dp[i-1][j], dp[i][j-1], dp[i-1][j-1])
        return dp[n][m]
    
```

### C++
```c++
class Solution {
public:
    int longestCommonSubsequence(string text1, string text2) {
        int n = text1.size();
        int m = text2.size();
        std::vector<vector<int>> dp(n + 1, vector<int>(m + 1));
        for (int j = 0; j <= m; ++j) {
            dp[0][j] = 0;
        }
        for (int i = 0; i <= n; ++i) {
            dp[i][0] = 0;
        }
        for (int i = 1; i <= n; ++i) {
            for (int j = 1; j <= m; ++j) {
                if (text1[i - 1] == text2[j - 1]) {
                    dp[i][j] = max3(dp[i-1][j-1] + 1, dp[i-1][j], dp[i][j - 1]);
                } else {
                    dp[i][j] = max3(dp[i-1][j-1], dp[i-1][j], dp[i][j-1]);
                }
            }
        }
        return dp[n][m];
    }
    int max3(int a, int b, int c) {
        int maxv = a;
        if (maxv < b) maxv = b;
        if (maxv < c) maxv = c;
        return maxv;
    }
};
```