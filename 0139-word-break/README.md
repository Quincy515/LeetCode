# 139. 单词拆分
给定一个**非空**字符串 *s* 和一个包含**非空**单词的列表 *wordDict*，判定 *s* 是否可以被空格拆分为一个或多个在字典中出现的单词。

#### 说明:
* 拆分时可以重复使用字典中的单词。
* 你可以假设字典中没有重复的单词。

#### 示例 1:
<pre>
<strong>输入:</strong> s = "leetcode", wordDict = ["leet","code"]
<strong>输出:</strong> true
<strong>解释:</strong> 返回 true 因为 "leetcode" 可以被拆分成 "leet code"。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s = "applepenapple", wordDict = ["apple","pen"]
<strong>输出:</strong> true
<strong>解释:</strong> 返回 true 因为 "applepenapple" 可以被拆分成 "apple pen apple"。
     注意你可以重复使用字典中的单词。
</pre>

#### 示例 3:
<pre>
<strong>输入:</strong> s = "catsandog", wordDict = ["cats","dog","sand","and","cat"]
<strong>输出:</strong> false
</pre>

## 题解 
### Rust
```rust
impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let n = s.len();
        // dp[i] 表示长度为 i 的字符串时可拆分的
        let mut dp = vec![false; n + 1];
        dp[0] = true;
        for i in 1..=n {
            for word in &word_dict {
                let len = word.len();
                if i >= len && s[i - len..i] == *word && dp[i - len] {
                    dp[i] = true;
                    break;
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

func wordBreak(s string, wordDict []string) bool {
	n := len(s)
	dp := make([]bool, n+1)
	dp[0] = true
	for i := 1; i <= n; i++ {
		for _, word := range wordDict {
			len := len(word)
			startp := i - len
			if startp >= 0 && startsWith(s, word, startp) && dp[i-len] {
				dp[i] = true
				break
			}
		}
	}
	return dp[n]
}

func startsWith(s, word string, start int) bool {
	return s[start:start+len(word)] == word
}

```

### JavaScript
```javascript
/**
 * @param {string} s
 * @param {string[]} wordDict
 * @return {boolean}
 */
var wordBreak = function(s, wordDict) {
    let n = s.length
    let dp = new Array(n+1).fill(false)
    dp[0] = true
    for (let i = 1; i <= n; i++) {
        for (let word of wordDict) {
            let len = word.length
            let startp = i - len
            if (startp >= 0 && s.startsWith(word, startp) && dp[i-len]) {
                dp[i] = true
                break
            }
        }
    }
    return dp[n]
};
```

### Python
```python
class Solution:
    def wordBreak(self, s: str, wordDict: List[str]) -> bool:
        n = len(s)
        dp = [False] * (n + 1)
        dp[0] = True
        for i in range(1, n+1):
            for word in wordDict:
                lenght = len(word)
                startp = i - lenght
                if startp >= 0 and s.startswith(word,startp) and dp[i - lenght]:
                    dp[i] = True
        return dp[n]

```