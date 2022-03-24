<p>给定一个仅包含数字&nbsp;<code>2-9</code>&nbsp;的字符串，返回所有它能表示的字母组合。答案可以按 <strong>任意顺序</strong> 返回。</p>

<p>给出数字到字母的映射如下（与电话按键相同）。注意 1 不对应任何字母。</p>

<p><img src="https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2021/11/09/200px-telephone-keypad2svg.png" style="width: 200px;" /></p>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>

<pre>
<strong>输入：</strong>digits = "23"
<strong>输出：</strong>["ad","ae","af","bd","be","bf","cd","ce","cf"]
</pre>

<p><strong>示例 2：</strong></p>

<pre>
<strong>输入：</strong>digits = ""
<strong>输出：</strong>[]
</pre>

<p><strong>示例 3：</strong></p>

<pre>
<strong>输入：</strong>digits = "2"
<strong>输出：</strong>["a","b","c"]
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>0 &lt;= digits.length &lt;= 4</code></li>
	<li><code>digits[i]</code> 是范围 <code>['2', '9']</code> 的一个数字。</li>
</ul>
<div><div>Related Topics</div><div><li>哈希表</li><li>字符串</li><li>回溯</li></div></div>

## 题解
### Rust
```rust
impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut result: Vec<String> = vec![];
        if digits.is_empty() {
            return result;
        }
        let mut mappings = vec!["".to_string(); 10];
        mappings[2] = "abc".to_string();
        mappings[3] = "def".to_string();
        mappings[4] = "ghi".to_string();
        mappings[5] = "jkl".to_string();
        mappings[6] = "mno".to_string();
        mappings[7] = "pqrs".to_string();
        mappings[8] = "tuv".to_string();
        mappings[9] = "wxyz".to_string();
        let mut path = vec![' '; digits.len()];
        let digits: Vec<char> = digits.chars().collect();
        Self::backtrack(&mappings, &digits, 0, &mut path, &mut result);
        result
    }
    fn backtrack(
        mappings: &Vec<String>,
        digits: &Vec<char>,
        k: i32,
        path: &mut Vec<char>,
        result: &mut Vec<String>,
    ) {
        if k as usize == digits.len() {
            result.push(path.iter().collect::<String>());
            return;
        }
        let mapping = mappings[digits[k as usize].to_digit(10).unwrap() as usize].clone();
        for c in mapping.chars() {
            path[k as usize] = c;
            Self::backtrack(mappings, digits, k + 1, path, result);
        }
    }
}
```

### Go
```go
var result []string

func letterCombinations(digits string) []string {
	result = make([]string, 0)
	if len(digits) == 0 {
		return []string{}
	}
	mappings := make([]string, 10)
	mappings[2] = "abc"
	mappings[3] = "def"
	mappings[4] = "ghi"
	mappings[5] = "jkl"
	mappings[6] = "mno"
	mappings[7] = "pqrs"
	mappings[8] = "tuv"
	mappings[9] = "wxyz"
	path := make([]byte, len(digits))
	backtrack(mappings, digits, 0, path)
	return result
}
func backtrack(mappings []string, digits string, k int, path []byte) {
	if k == len(digits) {
		result = append(result, string(path))
		return
	}
	mapping := mappings[digits[k]-'0']
	for i := 0; i < len(mapping); i++ {
		path[k] = mapping[i]
		backtrack(mappings, digits, k+1, path)
	}
}
```

### Python
```python
class Solution:
    def __init__(self):
        self.result = []

    def letterCombinations(self, digits: str) -> List[str]:
        if len(digits) == 0:
            return self.result
        mappings = [None] * 10
        mappings[2] = "abc"
        mappings[3] = "def"
        mappings[4] = "ghi"
        mappings[5] = "jkl"
        mappings[6] = "mno"
        mappings[7] = "pqrs"
        mappings[8] = "tuv"
        mappings[9] = "wxyz"
        path = [None] * len(digits)
        self.backtrack(mappings, digits, 0, path)
        return self.result

    def backtrack(self, mappings, digits, k, path):
        if k == len(digits):
            self.result.append("".join(path))
            return
        mapping = mappings[int(digits[k])]
        for i in range(len(mapping)):
            path[k] = mapping[i]
            self.backtrack(mappings, digits, k+1, path)

```

### JavaScript

```js
/**
 * @param {string} digits
 * @return {string[]}
 */
var letterCombinations = function(digits) {
    let result = []
    if (digits.length == 0) {
        return result
    }
    let mappings = new Array(10)
    mappings[2] = "abc"
    mappings[3] = "def"
    mappings[4] = "ghi"
    mappings[5] = "jkl"
    mappings[6] = "mno"
    mappings[7] = "pqrs"
    mappings[8] = "tuv"
    mappings[9] = "wxyz"
    let path = new Array(digits.length)
    let backtrack = k => {
        if (k == digits.length) {
            result.push(path.join(''))
            return
        }

        let mapping = mappings[digits.charAt(k).charCodeAt(0) - '0'.charCodeAt(0)]
        for (let i of mapping) {
            path[k] = i
            backtrack(k+1)
        }
    }
    backtrack(0)
    return result
};
```

### C++
```c++
class Solution {
public:
    vector<string> result;
    vector<string> letterCombinations(string digits) {
        if (digits.size() == 0) return {};
        vector<string> mappings(10);
        mappings[2] = "abc";
        mappings[3] = "def";
        mappings[4] = "ghi";
        mappings[5] = "jkl";
        mappings[6] = "mno";
        mappings[7] = "pqrs";
        mappings[8] = "tuv";
        mappings[9] = "wxyz";
        vector<char> path(digits.size());
        backtrack(mappings, digits, 0, path);
        return result;
    }
    void backtrack(vector<string> mappings, string digits, int k, vector<char> path) {
        if (k == digits.size()) {
            string str(path.begin(), path.end());
            result.push_back(str);
            return;
        }
        string mapping = mappings[digits[k] - '0'];
        for (int i = 0; i < mapping.size(); ++i) {
            path[k] = mapping[i];
            backtrack(mappings, digits, k + 1, path);
        }
    }
};
```