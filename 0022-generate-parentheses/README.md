# 22. 括号生成
数字 *n* 代表生成括号的对数，请你设计一个函数，用于能够生成所有可能的并且 **有效的** 括号组合。

#### 示例:
<pre>
<strong>输入:</strong> n = 3
<strong>输出:</strong> [
       "((()))",
       "(()())",
       "(())()",
       "()(())",
       "()()()"
     ]
</pre>

## 题解 

### Rust
```rust
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result: Vec<String> = vec![];
        Self::backtrack(n, 0, 0, 0, &mut vec![' '; 2 * n as usize], &mut result);
        result
    }

    fn backtrack(
        n: i32,
        left_used: i32,
        right_used: i32,
        k: i32,
        path: &mut Vec<char>,
        result: &mut Vec<String>,
    ) {
        if k == 2 * n {
            result.push(path[..].iter().collect::<String>());
            return;
        }
        if left_used < n {
            path[k as usize] = '(';
            Self::backtrack(n, left_used + 1, right_used, k + 1, path, result);
        }
        if left_used > right_used {
            path[k as usize] = ')';
            Self::backtrack(n, left_used, right_used + 1, k + 1, path, result);
        }
    }
}
```

### Go
```go
package main

var result []string

func generateParenthesis(n int) []string {
	result = make([]string, 0)
	path := make([]byte, 2*n)
	backtrack(n, 0, 0, 0, path)
	return result
}
func backtrack(n, leftUsed, rightUsed, k int, path []byte) {
	if k == 2*n {
		result = append(result, string(path))
		return
	}
	if leftUsed < n {
		path[k] = '('
		backtrack(n, leftUsed+1, rightUsed, k+1, path)
	}
	if leftUsed > rightUsed {
		path[k] = ')'
		backtrack(n, leftUsed, rightUsed+1, k+1, path)
	}
}

```

### JavaScript
```javascript
/**
 * @param {number} n
 * @return {string[]}
 */
var generateParenthesis = function(n) {
    let result = []
    let path = new Array(2*n)
    let backtrack = (leftUsed, rightUsed, k) => {
        if (k == 2*n) {
            result.push(path.join(""))
            return
        }
        if (leftUsed < n) {
            path[k] = '('
            backtrack(leftUsed+1, rightUsed, k+1)
        }
        if (leftUsed > rightUsed) {
            path[k] = ')'
            backtrack(leftUsed, rightUsed+1, k+1)
        }
    }
    backtrack(0,0,0)
    return result
};
```

### Python
```python
class Solution:
    def __init__(self):
        self.result = []

    def generateParenthesis(self, n: int) -> List[str]:
        path = [None] * 2 * n
        self.backtrack(n, 0, 0, 0, path)
        return self.result

    def backtrack(self, n, leftUsed, rightUesd, k, path):
        if k == 2*n:
            self.result.append("".join(path))
            return
        if leftUsed > rightUesd:
            path[k] = ")"
            self.backtrack(n, leftUsed, rightUesd+1, k+1, path)
        if leftUsed < n:
            path[k] = "("
            self.backtrack(n, leftUsed+1, rightUesd, k+1, path)

```

### C++
```c++
class Solution {
public:
    vector<string> result;
    vector<string> generateParenthesis(int n) {
        vector<char> path(2 * n);
        backtrack(n, 0, 0, 0 ,path);
        return result;
    }
    void backtrack(int n, int leftUsed, int rightUsed, int k, vector<char>& path) {
        if (k == 2 * n) {
            string str(path.begin(), path.end());
            result.push_back(str);
            return;
        }
        if (leftUsed < n) {
            path[k] = '(';
            backtrack(n, leftUsed + 1, rightUsed, k + 1, path);
        }
        if (leftUsed > rightUsed) {
            path[k] = ')';
            backtrack(n, leftUsed, rightUsed + 1, k + 1, path);
        }
    }
};
```