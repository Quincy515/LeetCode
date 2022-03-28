# 93. 复原IP地址
给定一个只包含数字的字符串，复原它并返回所有可能的 IP 地址格式。

有效的 IP 地址正好由四个整数（每个整数位于 0 到 255 之间组成），整数之间用 `'.'` 分隔。

#### 示例:
<pre>
<strong>输入:</strong> "25525511135"
<strong>输出:</strong> ["255.255.11.135", "255.255.111.35"]
</pre>

## 题解 

### Rust
```rust
impl Solution {
    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let mut result: Vec<String> = vec![];
        let s = s.chars().collect::<Vec<char>>();
        Self::backtrack(&s, 0, 0, &mut vec![], &mut result);

        result
    }

    fn backtrack(s: &[char], k: i32, step: i32, path: &mut Vec<i32>, result: &mut Vec<String>) {
        if k == s.len() as i32 && step == 4 {
            let mut sb = String::new();
            for i in 0..3 {
                sb.push_str(&path[i].to_string());
                sb.push('.');
            }
            sb.push_str(&path[3].to_string());
            result.push(sb);
            return;
        }
        if step > 4 {
            return;
        }
        if k == s.len() as i32 {
            return;
        }
        let mut val = 0;
        // 1 位数
        if k < s.len() as i32 {
            val = val * 10 + (s[k as usize] as u8 - b'0') as i32;
            path.push(val);
            Self::backtrack(s, k + 1, step + 1, path, result);
            path.pop();
        }
        if s[k as usize] == '0' {
            // 前导0不行
            return;
        }
        // 2 位数
        if k + 1 < s.len() as i32 {
            val = val * 10 + (s[k as usize + 1] as u8 - b'0') as i32;
            path.push(val);
            Self::backtrack(s, k + 2, step + 1, path, result);
            path.pop();
        }
        // 3 位数
        if k + 2 < s.len() as i32 {
            val = val * 10 + (s[k as usize + 2] as u8 - b'0') as i32;
            if val <= 255 {
                path.push(val);
                Self::backtrack(s, k + 3, step + 1, path, result);
                path.pop();
            }
        }
    }
}
```

### Go
```go
package main

import (
	"fmt"
	"strings"
)

var result []string

func restoreIpAddresses(s string) []string {
	result = make([]string, 0)
	backtrack(s, 0, 0, []int{})
	return result
}
func backtrack(s string, k, step int, path []int) {
	if k == len(s) && step == 4 {
		sb := strings.Builder{}
		for i := 0; i < 3; i++ {
			sb.Write([]byte(fmt.Sprintf("%d", path[i])))
			sb.WriteByte(byte('.'))
		}
		sb.Write([]byte(fmt.Sprintf("%d", path[3])))
		result = append(result, sb.String())
	}
	if step > 4 {
		return
	}
	if k == len(s) {
		return
	}
	val := 0 //1 位数
	if k < len(s) {
		val = val*10 + int(s[k]-'0')
		path = append(path, val)
		backtrack(s, k+1, step+1, path)
		path = path[:len(path)-1]
	}
	if s[k] == '0' {
		return
	} //2 位数
	if k+1 < len(s) {
		val = val*10 + int(s[k+1]-'0')
		path = append(path, val)
		backtrack(s, k+2, step+1, path)
		path = path[:len(path)-1]
	}
	//3 位数
	if k+2 < len(s) {
		val = val*10 + int(s[k+2]-'0')
		if val <= 255 {
			path = append(path, val)
			backtrack(s, k+3, step+1, path)
			path = path[:len(path)-1]
		}
	}
}

```

### JavaScript
```javascript
/**
 * @param {string} s
 * @return {string[]}
 */
var restoreIpAddresses = function(s) {
    let result = []
    let path = []
    let backtrack = (k, step) => {
        if (k == s.length && step == 4) {
            result.push(path.join("."))
            return
        }
        if (step > 4) {
            return
        }
        if (k == s.length) {
            return
        }
        let val = 0
        if (k < s.length) {
            val = val * 10 + (s[k].charCodeAt(0)-'0'.charCodeAt(0))
            path.push(val)
            backtrack(k + 1, step + 1)
            path.pop()
        }
        if (s[k] == '0') {
            return
        }
        if (k+1 < s.length) {
            val = val * 10 + (s[k+1].charCodeAt(0) - '0'.charCodeAt(0))
            path.push(val)
            backtrack(k+2, step+1)
            path.pop()
        }
        if (k+2 < s.length) {
            val = val * 10 + (s[k+2].charCodeAt(0) - '0'.charCodeAt(0))
            if (val <= 255) {
                path.push(val)
                backtrack(k+3, step+1)
                path.pop()
            }
        }
    }
    backtrack(0, 0)
    return result
};
```

### Python
```python
class Solution:
    def __init__(self):
        self.result = []

    def restoreIpAddresses(self, s: str) -> List[str]:
        self.backtrack(s, 0,0, [])
        return self.result

    def backtrack(self, s, k, step, path):
        if step == 4 and k == len(s):
            sb = [str(item) for item in path]
            self.result.append(".".join(sb))
            return
        if step > 4:
            return
        if k == len(s):
            return
        val = 0
        if k < len(s):
            val = val * 10 + int(s[k])
            path.append(val)
            self.backtrack(s, k+1, step+1, path)
            path.pop()
        if s[k] == "0":
            return
        if k + 1 < len(s):
            val = val * 10 + int(s[k+1])
            path.append(val)
            self.backtrack(s, k+2, step+1, path)
            path.pop()
        if k + 2 < len(s):
            val = val * 10 + int(s[k+2])
            if val <= 255:
                path.append(val)
                self.backtrack(s, k+3, step+1, path)
                path.pop()

```

### C++
```c++
class Solution {
public:
    vector<string> result;
    vector<string> restoreIpAddresses(string s) {
        vector<int> path;
        backtrack(s, 0, 0, path);
        return result;
    }
    void backtrack(string s, int k, int step, vector<int> &path) {
        if (k == s.size() && step == 4) {
            string sb;
            for (int i = 0; i < 3; ++i) {
                sb += to_string(path[i]);
                sb += ".";
            }
            sb += to_string(path[3]);
            result.push_back(sb);
            return;
        }
        if (step > 4) {
            return;
        }
        if (k == s.size()) {
            return;
        }
        int val = 0;
        if (k < s.size()) {
            val = val * 10 + (s[k] - '0');
            path.push_back(val);
            backtrack(s, k + 1, step + 1, path);
            path.pop_back();
        }
        if (s[k] == '0') {
            return;
        }
        if (k + 1 < s.size()) {
            val = val * 10 + (s[k + 1] - '0');
            path.push_back(val);
            backtrack(s, k + 2, step + 1, path);
            path.pop_back();
        }
        if (k + 2 < s.size()) {
            val = val * 10 + (s[k+2] - '0');
            if (val <= 255) {
                path.push_back(val);
                backtrack(s, k + 3, step + 1, path);
                path.pop_back();
            }
        }
    }
};
```