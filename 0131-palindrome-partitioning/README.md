<p>给你一个字符串 <code>s</code>，请你将<em> </em><code>s</code><em> </em>分割成一些子串，使每个子串都是 <strong>回文串</strong> 。返回 <code>s</code> 所有可能的分割方案。</p>

<p><strong>回文串</strong> 是正着读和反着读都一样的字符串。</p>

<p> </p>

<p><strong>示例 1：</strong></p>

<pre>
<strong>输入：</strong>s = "aab"
<strong>输出：</strong>[["a","a","b"],["aa","b"]]
</pre>

<p><strong>示例 2：</strong></p>

<pre>
<strong>输入：</strong>s = "a"
<strong>输出：</strong>[["a"]]
</pre>

<p> </p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 <= s.length <= 16</code></li>
	<li><code>s</code> 仅由小写英文字母组成</li>
</ul>
<div><div>Related Topics</div><div><li>字符串</li><li>动态规划</li><li>回溯</li></div></div><br>

## 题解
### Rust
```rust
impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut result: Vec<Vec<String>> = vec![];
        let s = s.chars().collect::<Vec<char>>();
        Self::backtrack(&s, 0, &mut vec![], &mut result);
        result
    }

    fn backtrack(s: &[char], k: i32, path: &mut Vec<String>, result: &mut Vec<Vec<String>>) {
        if k == s.len() as i32 {
            result.push(path.clone());
            return;
        }
        for end in k..s.len() as i32 {
            if Self::ispalidrome(s, k, end) {
                path.push(s[k as usize..=end as usize].iter().collect::<String>());
                Self::backtrack(s, end + 1, path, result);
                path.pop();
            }
        }
    }
    fn ispalidrome(s: &[char], p: i32, r: i32) -> bool {
        let mut i = p;
        let mut j = r;
        while i <= j {
            if s[i as usize] != s[j as usize] {
                return false;
            }
            i += 1;
            j -= 1;
        }
        true
    }
}
```

### Go
```go
package main

var result [][]string

func partition(s string) [][]string {
	result = make([][]string, 0)
	backtrack(s, 0, []string{})
	return result
}
func backtrack(s string, k int, path []string) {
	if k == len(s) {
		snapshot := make([]string, len(path))
		copy(snapshot, path)
		result = append(result, snapshot)
		return
	}
	for end := k; end < len(s); end++ {
		if ispalidrome(s, k, end) {
			path = append(path, s[k:end+1])
			backtrack(s, end+1, path)
			path = path[:len(path)-1]
		}
	}
}
func ispalidrome(s string, p int, r int) bool {
	i := p
	j := r
	for i <= j {
		if s[i] != s[j] {
			return false
		}
		i++
		j--
	}
	return true
}

```

### JavaScript
```javascript
/**
 * @param {string} s
 * @return {string[][]}
 */
var partition = function(s) {
    let result = []
    let path = []
    let backtrack = k => {
        if (k == s.length) {
            result.push([...path])
            return
        }
        for (let end = k; end < s.length; ++end) {
            if (ispalindrome(s, k, end)) {
                path.push(s.substring(k, end+1))
                backtrack(end+1)
                path.pop()
            }
        }
    }
    backtrack(0)
    return result
};
var ispalindrome = (s, p, r) => {
    let i = p
    let j = r
    while (i <= j) {
        if (s[i] != s[j]) {
            return false
        }
        i++
        j--
    }
    return true
}
```

### Python
```python
class Solution:
    def __init__(self):
        self.result = []

    def partition(self, s: str) -> List[List[str]]:
        self.backtrack(s, 0, [])
        return self.result

    def backtrack(self, s, k, path):
        if k == len(s):
            self.result.append(path[:])
            return
        for end in range(k, len(s)):
            if self.ispalindrome(s, k, end):
                path.append(s[k:end+1])
                self.backtrack(s, end+1, path)
                path.pop()

    def ispalindrome(self, s, p, r):
        i = p
        j = r
        while i <= j:
            if s[i] !=s[j]:
                return False
            i += 1
            j -= 1
        return True
```

### C++
```c++
class Solution {
public:
    vector<vector<string>> result;
    vector<vector<string>> partition(string s) {
        vector<string> path;
        backtrack(s, 0, path);
        return result;
    }
    void backtrack(string s, int k, vector<string>& path) {
        if (k == s.size()) {
            result.push_back(path);
            return;
        }
        for (int end = k; end < s.size(); ++end) {
            if (ispalindrome(s, k, end)) {
                path.push_back(s.substr(k, end + 1 - k));
                backtrack(s, end + 1, path);
                path.pop_back();
            }
        }
    }
    bool ispalindrome(string s, int p, int r) {
        int i = p;
        int j = r;
        while (i <= j) {
            if (s[i] != s[j]) return false;
            i++;
            j--;
        }
        return true;
    }
};
```