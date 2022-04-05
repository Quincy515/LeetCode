# 76. 最小覆盖子串

给你一个字符串 s 、一个字符串 t 。返回 s 中涵盖 t 所有字符的最小子串。如果 s 中不存在涵盖 t 所有字符的子串，则返回空字符串 "" 。

## 注意：

- 对于 t 中重复字符，我们寻找的子字符串中该字符数量必须不少于 t 中该字符数量。
- 如果 s 中存在这样的子串，我们保证它是唯一的答案。


## 示例 1：
```
输入：s = "ADOBECODEBANC", t = "ABC"
输出："BANC"
```

## 示例 2：
```
输入：s = "a", t = "a"
输出："a"
```

## 示例 3:
```
输入: s = "a", t = "aa"
输出: ""
解释: t 中两个字符 'a' 均应包含在 s 的子串中，
因此没有符合条件的子字符串，返回空字符串。
```

## 提示：

- 1 <= s.length, t.length <= 105
- s 和 t 由英文字母组成


## 进阶
你能设计一个在 o(n) 时间内解决此问题的算法吗？

## 题解：
### Rust
```rust

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let (s, t) = (
            s.chars().collect::<Vec<char>>(),
            t.chars().collect::<Vec<char>>(),
        );
        let (mut min_wsize, mut min_wstart, mut min_wend) = (i32::MAX, -1i32, -1i32);
        // 模式串、滑动窗口
        let (mut tmap, mut wmap) = (HashMap::new(), HashMap::new());
        for c in t {
            let mut count = 1;
            // *tmap.entry(c).or_insert(0) += 1;
            if tmap.contains_key(&c) {
                count += tmap.get(&c).unwrap();
            }
            tmap.insert(c, count);
        }
        let (n, mut l, mut r) = (s.len() as i32, 0i32, -1i32);
        while l < n && r < n {
            while !Self::rmatch(&wmap, &tmap) {
                r += 1;
                if r > n - 1 {
                    break;
                }
                let c = s[r as usize];
                if tmap.contains_key(&c) {
                    let mut count = 1;
                    if wmap.contains_key(&c) {
                        count += wmap[&c];
                    }
                    wmap.insert(c, count);
                }
            }
            if Self::rmatch(&wmap, &tmap) {
                if min_wsize > r - l + 1 {
                    min_wsize = r - l + 1;
                    min_wstart = l;
                    min_wend = r;
                }
                let c = s[l as usize];
                if tmap.contains_key(&c) {
                    let count = wmap[&c];
                    if count - 1 == 0 {
                        wmap.remove(&c);
                    } else {
                        wmap.insert(c, count - 1);
                    }
                }
                l += 1;
            }
        }

        if min_wstart == -1 {
            return "".to_string();
        }
        s[min_wstart as usize..min_wend as usize + 1]
            .iter()
            .collect::<String>()
    }

    fn rmatch(wmap: &HashMap<char, i32>, tmap: &HashMap<char, i32>) -> bool {
        for (c, count) in tmap {
            if !wmap.contains_key(c) {
                return false;
            }
            if wmap.get(c).unwrap() < count {
                return false;
            }
        }
        true
    }
}
```

### Go
```go

package main

import "math"

func minWindow(s string, t string) string {
	minWSize := math.MaxInt32
	minWStart := -1
	minWEnd := -1
	tmap := make(map[byte]int, 0) //模式串
	wmap := make(map[byte]int, 0) //滑动窗口
	for i := 0; i < len(t); i++ {
		count := 1
		if value, ok := tmap[t[i]]; ok {
			count += value
		}
		tmap[t[i]] = count
	}
	n := len(s)
	l := 0
	r := -1
	for l < n && r < n {
		for !match(wmap, tmap) {
			r++
			if r > n-1 {
				break
			}
			c := s[r]
			if _, ok := tmap[c]; ok {
				count := 1
				if _, ok := wmap[c]; ok {
					count += wmap[c]
				}
				wmap[c] = count
			}
		}
		if match(wmap, tmap) {
			if minWSize > r-l+1 {
				minWSize = r - l + 1
				minWStart = l
				minWEnd = r
			}
			c := s[l]
			if _, ok := tmap[c]; ok {
				count := wmap[c]
				if count-1 == 0 {
					delete(wmap, c)
				} else {
					wmap[c] = count - 1
				}
			}
			l++
		}
	}
	if minWStart == -1 {
		return ""
	}
	return s[minWStart : minWEnd+1]
}
func match(wmap, tmap map[byte]int) bool {
	for key, value := range tmap {
		if _, ok := wmap[key]; !ok {
			return false
		}
		if wmap[key] < value {
			return false
		}
	}
	return true
}
```

### JavaScript
```javascript
/**
 * @param {string} s
 * @param {string} t
 * @return {string}
 */
var minWindow = function(s, t) {
    let minWSize = Number.MAX_SAFE_INTEGER
    let minWSStart = -1
    let minWEnd = -1
    let tmap = new Map()
    let wmap = new Map()
    for (let i = 0; i < t.length; ++i) {
        let count = 1
        if (tmap.has(t[i])) {
            count += tmap.get(t[i])
        }
        tmap.set(t[i], count)
    }
    let n = s.length
    let l = 0
    let r = -1
    while (l < n && r < n) {
        while (!match(wmap, tmap)) {
            r++
            if (r > n-1) {
                break
            }
            let c = s[r]
            if (tmap.has(c)) {
                let count = 1
                if (wmap.has(c)) {
                    count += wmap.get(c)
                }
                wmap.set(c, count)
            }
        }
        if (match(wmap, tmap)) {
            if (minWSize > r-l+1) {
                minWSize = r-l+1
                minWSStart = l
                minWEnd = r
            }
            let c = s[l]
            if (tmap.has(c)) {
                let count = wmap.get(c)
                if (count - 1 == 0) {
                    wmap.delete(c)
                } else {
                    wmap.set(c, count-1)
                }
            }
            l++
        }
    }
    if (minWSStart == -1) {
        return ""
    }
    return s.substring(minWSStart, minWEnd+1)

};
var match = (wmap, tmap) => {
    for (let [key, value] of tmap) {
        if (!wmap.has(key)) {
            return false
        }
        if (wmap.get(key) < value) {
            return false
        }
    }
    return true
}
```

### Python
```python
class Solution:
    def minWindow(self, s: str, t: str) -> str:
        minWSize = float("inf")
        minWStart = -1
        minWEnd = -1
        tmap = dict()
        wmap = dict()
        for i in range(len(t)):
            count = 1
            if t[i] in tmap:
                count += tmap[t[i]]
            tmap[t[i]] = count
        n = len(s)
        l = 0
        r = -1
        while l < n and r < n:
            while not self.match(wmap, tmap):
                r += 1
            if r > n - 1:
                break
            c = s[r]
            if c in tmap:
                count = 1
                if c in wmap:
                    count += wmap[c]
                wmap[c] = count
        if self.match(wmap, tmap):
            if minWSize > r - l + 1:
                minWSize = r - l + 1
                minWStart = l
                minWEnd = r
            c = s[l]
            if c in tmap:
                count = wmap[c]
                if count - 1 == 0:
                    wmap.pop(c)
                else:
                    wmap[c] -= 1
            l += 1
        if minWStart == -1:
            return ""
        return s[minWStart:minWEnd+1]
    def match(self, wmap, tmap):
        for key, val in tmap.items():
            if key not in wmap:
                return False
            if val > wmap[key]:
                return False
        return True

```

### C++
```c++
class Solution {
public:
    string minWindow(string s, string t) {
        int minWSize = INT_MAX;
        int minWStart = -1;
        int minWEnd = -1;
        unordered_map<char, int> tmap;
        unordered_map<char, int> wmap;
        for (int i = 0; i < t.size(); ++i) {
            int count = 1;
            if (tmap.find(t[i]) != tmap.end()) {
                count += tmap[t[i]];
            }
            tmap[t[i]] = count;
        }

        int n = s.size();
        int l = 0;
        int r = -1;
        while (l < n && r < n) {
            while (!match(wmap, tmap)) {
                r++;
                if (r > n - 1) {
                    break;
                }
                char c = s[r];
                if (tmap.find(c) != tmap.end()) {
                    int count = 1;
                    if (wmap.find(c) != wmap.end()) {
                        count += wmap[c];
                    }
                    wmap[c] = count;
                }
            }
            if (match(wmap, tmap)) {
                if (minWSize > r - l + 1) {
                    minWSize = r - l + 1;
                    minWStart = l;
                    minWEnd = r;
                }
                char c = s[l];
                if (tmap.find(c) != tmap.end()) {
                    int count = wmap[c];
                    if (count - 1 == 0) {
                        wmap.erase(c);
                    } else {
                        wmap[c] = count - 1;
                    }
                }
                l++;
            }
        }
        if (minWStart == -1) return "";
        return s.substr(minWStart, minWEnd + 1 - minWStart);
    }
    bool match(unordered_map<char, int>& wmap, unordered_map<char, int>& tmap) {
        for (auto it = tmap.begin(); it != tmap.end(); ++it) {
            char key = it->first;
            if (wmap.find(key) == wmap.end()) return false;
            if (wmap[key] < it->second) return false;
        }
        return true;
    }
};

```