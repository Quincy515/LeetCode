# 438. 找到字符串中所有字母异位词
给定一个字符串 **s** 和一个非空字符串 **p**，找到 **s** 中所有是 **p** 的字母异位词的子串，返回这些子串的起始索引。

字符串只包含小写英文字母，并且字符串 **s** 和 **p** 的长度都不超过 20100。

#### 说明:
* 字母异位词指字母相同，但排列不同的字符串。
* 不考虑答案输出的顺序。

#### 示例 1:
<pre>
<strong>输入:</strong> s: "cbaebabacd" p: "abc"
<strong>输出:</strong> [0, 6]
<strong>解释:</strong>
起始索引等于 0 的子串是 "cba", 它是 "abc" 的字母异位词。
起始索引等于 6 的子串是 "bac", 它是 "abc" 的字母异位词。
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> s: "abab" p: "ab"
<strong>输出:</strong> [0, 1, 2]
<strong>解释:</strong>
起始索引等于 0 的子串是 "ab", 它是 "ab" 的字母异位词。
起始索引等于 1 的子串是 "ba", 它是 "ab" 的字母异位词。
起始索引等于 2 的子串是 "ab", 它是 "ab" 的字母异位词。
</pre>

## 题解
### Rust
```rust
impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let (s, p) = (s.as_bytes(), p.as_bytes());
        let (n, m) = (s.len(), p.len());
        if m > n {
            return vec![];
        }
        let mut needs = vec![0; 26];
        for i in 0..m {
            needs[(p[i] - b'a') as usize] += 1;
        }
        let mut matched = vec![0; 26];

        let (mut startp, mut endp) = (0, 0);
        let mut result = vec![];
        while endp < m {
            matched[(s[endp] - b'a') as usize] += 1;
            endp += 1;
        }
        if Self::same(&needs, &matched) {
            result.push(startp as i32);
        }
        while endp < n && startp < n {
            matched[(s[startp] - b'a') as usize] -= 1;
            matched[(s[endp] - b'a') as usize] += 1;
            startp += 1;
            endp += 1;
            if Self::same(&needs, &matched) {
                result.push(startp as i32);
            }
        }
        result
    }
    fn same(needs: &[i32], matched: &[i32]) -> bool {
        for i in 0..needs.len() {
            if needs[i] != matched[i] {
                return false;
            }
        }
        true
    }
}

```

### Go
```go
impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let (s, p) = (s.as_bytes(), p.as_bytes());
        let (n, m) = (s.len(), p.len());
        if m > n {
            return vec![];
        }
        let mut needs = vec![0; 26];
        for i in 0..m {
            needs[(p[i] - b'a') as usize] += 1;
        }
        let mut matched = vec![0; 26];

        let (mut startp, mut endp) = (0, 0);
        let mut result = vec![];
        while endp < m {
            matched[(s[endp] - b'a') as usize] += 1;
            endp += 1;
        }
        if Self::same(&needs, &matched) {
            result.push(startp as i32);
        }
        while endp < n && startp < n {
            matched[(s[startp] - b'a') as usize] -= 1;
            matched[(s[endp] - b'a') as usize] += 1;
            startp += 1;
            endp += 1;
            if Self::same(&needs, &matched) {
                result.push(startp as i32);
            }
        }
        result
    }
    fn same(needs: &[i32], matched: &[i32]) -> bool {
        for i in 0..needs.len() {
            if needs[i] != matched[i] {
                return false;
            }
        }
        true
    }
}

```

### JavaScript
```javascript
/**
 * @param {string} s
 * @param {string} p
 * @return {number[]}
 */
var findAnagrams = function(s, p) {
        let n = s.length
        let m = p.length
        if (m > n) {
            return []
        }
        let needs = new Array(26).fill(0)
        for (let i = 0; i < m; ++i) {
            needs[charCodeSubA(p[i])]++
        }
        let matched = new Array(26).fill(0)
        let startp = 0
        let endp = 0
        let result = []
        while (endp < m) {
            matched[charCodeSubA(s[endp])]++
            endp++
        }
        if (same(needs, matched)) {
            result.push(startp)
        }
        while (endp < n && startp < n) {
            matched[charCodeSubA(s[startp])]--
            matched[charCodeSubA(s[endp])]++
            startp++
            endp++
            if (same(needs, matched)) {
                result.push(startp)
            }
        }
        return result
    };
var same = (needs, matched) => {
    for (let i = 0; i < needs.length; ++i) {
        if (needs[i] !== matched[i]) {
            return false
        }
    }
    return true
}
var charCodeSubA = a => a.charCodeAt(0) - 'a'.charCodeAt(0)

```

### Python
```python
class Solution:
    def findAnagrams(self, s: str, p: str) -> List[int]:
        n = len(s)
        m = len(p)
        if m > n:
            return []
        needs = [0] * 26
        for i in range(m):
            needs[ord(p[i]) - ord("a")] +=1
        matched = [0] * 26
        startp = 0
        endp = 0
        result = []
        while endp < m:
            matched[ord(s[endp]) - ord("a")] += 1
            endp += 1
        if self.same(needs, matched):
            result.append(startp)
        while endp < n and startp < n:
            matched[ord(s[startp]) - ord("a")] -= 1
            matched[ord(s[endp]) - ord("a")] += 1
            startp += 1
            endp += 1
            if self.same(needs, matched):
                result.append(startp)
        return result

    def same(self, needs, matched):
        for i in range(len(needs)):
            if needs[i] != matched[i]:
                return False
        return True

```

### C++
```c++
class Solution {
public:
    vector<int> findAnagrams(string s, string p) {
        int n = s.size();
        int m = p.size();
        if (m > n) return {};
        vector<int> needs(26);
        for (int i = 0; i < m; ++i) {
            needs[p[i] - 'a']++;
        }
        vector<int> matched(26);
        int startp = 0;
        int endp = 0;
        vector<int> result;
        while (endp < m) {
            matched[s[endp] - 'a']++;
            endp++;
        }
        if (same(needs, matched)) {
            result.push_back(startp);
        }
        while (endp < n && startp < n) {
            matched[s[startp] - 'a']--;
            matched[s[endp] - 'a']++;
            startp++;
            endp++;
            if (same(needs, matched)) {
                result.push_back(startp);
            }
        }
        return result;
    }
    bool same(const vector<int>& needs, const vector<int>& matched) {
        for (int i = 0; i < needs.size(); ++i) {
            if (needs[i] != matched[i]) return false;
        }
        return true;
    }
}
```